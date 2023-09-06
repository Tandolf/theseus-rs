use fxhash::FxHashMap;
use image::{Pixel, RgbImage};
use std::{io::Error, iter};

use crate::node::{Direction, Node, NodeType, Point};
use crate::utils::{look_ahead, path_above, path_below, wall_above, wall_below};

#[derive(Debug)]
pub struct Maze {
    pub width: u32,
    pub height: u32,
    pub data: Nodes,
}

pub type Nodes = FxHashMap<NodeType, Node>;

pub const WALL: [u8; 3] = [0, 0, 0];
pub const PATHWAY: [u8; 3] = [255, 255, 255];

// X = columns
// y = rows

fn get_entrance(image: &RgbImage, nodes: &mut Nodes, top_nodes: &mut [Option<Point>]) {
    for (x, y, pixel) in image.enumerate_pixels() {
        if let [255, 255, 255] = pixel.channels() {
            let node = Node::start(x, y);
            nodes.insert(NodeType::Start, node);
            top_nodes[x as usize] = Some(Point::at(x, y));
            break;
        }
    }
}

fn get_exit<'a>(image: &RgbImage, nodes: &'a mut Nodes, top_nodes: &'a [Option<Point>]) {
    let last_row = image.height() - 1;
    for (x, pixel) in image.rows().last().unwrap().enumerate() {
        if pixel.channels() == PATHWAY {
            let above = top_nodes[x].as_ref().unwrap();
            let current = Point::at(x as u32, last_row);

            let mut exit_node = Node::exit(current.x, current.y);
            exit_node.children.insert(Direction::North, *above);
            nodes.insert(NodeType::Exit, exit_node);

            match nodes.get_mut(&NodeType::Path(*above)) {
                Some(node_above) => {
                    node_above.children.insert(Direction::South, current);
                }
                None => {
                    let node_above = nodes.get_mut(&NodeType::Start).unwrap();
                    node_above.children.insert(Direction::South, current);
                }
            }
            break;
        }
    }
}

impl Maze {
    pub(crate) fn from_image(image: &RgbImage) -> Result<Maze, Error> {
        let len = image.pixels().len();
        let mut nodes = FxHashMap::with_capacity_and_hasher(len / 6, Default::default());

        let width = &image.width() - 1;
        let mut top_nodes: Vec<Option<Point>> =
            iter::repeat_with(|| None).take(width as usize).collect();
        get_entrance(image, &mut nodes, &mut top_nodes);

        let (width, height) = image.dimensions();
        if height == 1 && nodes.len() == 1 {
            return Ok(Maze {
                width,
                height,
                data: nodes,
            });
        }

        if height > 2 {
            for y in 1..height - 1 {
                let mut current = false;
                let mut next = look_ahead(0, y, image);

                let mut left_node = None;

                for x in 1..width - 1 {
                    let prev = current;
                    current = next;
                    next = look_ahead(x, y, image);
                    let mut n = None;

                    if !current {
                        continue;
                    }

                    if prev {
                        if next {
                            // PATH PATH PATH
                            // only create node if path above or below
                            // check above or below
                            if path_above(x, y, image) || path_below(x, y, image) {
                                let mut node = Node::at(x, y);
                                node.children.insert(Direction::West, left_node.unwrap());

                                let left = left_node.unwrap();
                                nodes
                                    .get_mut(&NodeType::Path(left))
                                    .unwrap()
                                    .children
                                    .insert(Direction::East, Point::at(x, y));

                                nodes.insert(NodeType::Path(node.point), node);
                                left_node = Some(Point::at(x, y));
                                n = left_node;
                            }
                        } else {
                            // PATH PATH WALL
                            // Create Node at end of corridor
                            let mut node = Node::at(x, y);
                            node.children.insert(Direction::West, left_node.unwrap());

                            let left = left_node.unwrap();
                            nodes
                                .get_mut(&NodeType::Path(left))
                                .unwrap()
                                .children
                                .insert(Direction::East, Point::at(x, y));

                            nodes.insert(NodeType::Path(node.point), node);
                            left_node = None;
                            n = Some(Point::at(x, y));
                        }
                    } else if next {
                        // WALL PATH PATH
                        // Create path at start of corridor
                        let node = Node::at(x, y);
                        nodes.insert(NodeType::Path(node.point), node);
                        left_node = Some(Point::at(x, y));
                        n = Some(Point::at(x, y));
                    } else {
                        // WALL PATH WALL
                        // Only create if in a dead end
                        if wall_above(x, y, image) || wall_below(x, y, image) {
                            let node = Node::at(x, y);
                            nodes.insert(NodeType::Path(node.point), node);
                            n = Some(Point::at(x, y));
                        }
                    }

                    // If we have a node stored we can check if it needs to be connected upwards.
                    if let Some(current_n) = n {
                        // If path above, then there must be a Node to connect to above
                        if path_above(x, y, image) {
                            let top = top_nodes[x as usize].unwrap();

                            // If we dont find a path node it has to be the start node above us
                            match nodes.get_mut(&NodeType::Path(top)) {
                                Some(n) => {
                                    n.children.insert(Direction::South, current_n);
                                }
                                None => {
                                    nodes
                                        .get_mut(&NodeType::Start)
                                        .unwrap()
                                        .children
                                        .insert(Direction::South, current_n);
                                }
                            }

                            nodes
                                .get_mut(&NodeType::Path(Point::at(x, y)))
                                .unwrap()
                                .children
                                .insert(Direction::North, top);
                        }

                        // If clear below, then this will probably be connectable, so place it in the top row
                        if path_below(x, y, image) {
                            top_nodes[x as usize] = Some(current_n);
                        } else {
                            top_nodes[x as usize] = None;
                        }
                    }
                }
            }
        }

        if height > 1 {
            get_exit(image, &mut nodes, &top_nodes);
        }

        Ok(Maze {
            width: image.width(),
            height: image.height(),
            data: nodes,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use image::Rgb;

    const WALL: Rgb<u8> = Rgb([0, 0, 0]);
    const PATH: Rgb<u8> = Rgb([255, 255, 255]);

    macro_rules! maze_image {
        ($num:expr) => {{
            let mut img = RgbImage::new($num[0].len() as u32, $num.len() as u32);
            for (y, row) in $num.iter().enumerate() {
                for (x, item) in row.iter().enumerate() {
                    // if path
                    if *item == 1 {
                        img.put_pixel(x as u32, y as u32, PATH);
                    // if wall
                    } else {
                        img.put_pixel(x as u32, y as u32, WALL);
                    }
                }
            }
            img
        }};
    }
    //     // first maze row
    //     // xox
    #[test]
    fn entrance() {
        let img = maze_image!([[0, 1, 0]]);
        let maze_tree = Maze::from_image(&img).unwrap();

        let node = &maze_tree.data[&NodeType::Start];
        assert_eq!(&Node::start(1, 0), node)
    }

    // two maze rows, second only walls
    // xox
    // xxx
    #[test]
    fn entry_and_exit() {
        #[rustfmt::skip]
        let img = maze_image!([
            [0, 1, 0], 
            [0, 1, 0]
        ]);

        let maze_tree = Maze::from_image(&img).unwrap();

        let mut start = Node::start(1, 0);
        start.children.insert(Direction::South, Point::at(1, 1));

        let mut exit = Node::exit(1, 1);
        exit.children.insert(Direction::North, Point::at(1, 0));

        assert_eq!(&start, &maze_tree.data[&NodeType::Start]);
        assert_eq!(&exit, &maze_tree.data[&NodeType::Exit]);
    }

    // two maze rows, one middle path
    // xox
    // xox
    // xox
    #[test]
    fn first_corridor() {
        #[rustfmt::skip]
        let img = maze_image!([
            [0, 1, 0],
            [0, 1, 0],
            [0, 1, 0]
        ]);

        let maze_tree = Maze::from_image(&img).unwrap();

        let mut n1 = Node::start(1, 0);
        n1.children.insert(Direction::South, Point::at(1, 2));

        let mut n2 = Node::exit(1, 2);
        n2.children.insert(Direction::North, Point::at(1, 0));

        assert_eq!(&n1, maze_tree.data.get(&NodeType::Start).unwrap());
        assert_eq!(&n2, maze_tree.data.get(&NodeType::Exit).unwrap());
    }

    #[test]
    fn side_path() {
        #[rustfmt::skip]
        let img = maze_image!([
            [0, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 1, 0, 0]
        ]);

        let maze_tree = Maze::from_image(&img).unwrap();

        let mut n1 = Node::start(1, 0);
        n1.children.insert(Direction::South, Point::at(1, 1));

        let mut n2 = Node::at(1, 1);
        n2.children.insert(Direction::North, Point::at(1, 0));
        n2.children.insert(Direction::East, Point::at(2, 1));
        n2.children.insert(Direction::South, Point::at(1, 2));

        let mut n3 = Node::at(2, 1);
        n3.children.insert(Direction::West, Point::at(1, 1));

        let mut n4 = Node::exit(1, 2);
        n4.children.insert(Direction::North, Point::at(1, 1));

        assert_eq!(&n1, maze_tree.data.get(&NodeType::Start).unwrap());
        assert_eq!(
            &n2,
            maze_tree
                .data
                .get(&NodeType::Path(Point::at(1, 1)))
                .unwrap()
        );
        assert_eq!(
            &n3,
            maze_tree
                .data
                .get(&NodeType::Path(Point::at(2, 1)))
                .unwrap()
        );
        assert_eq!(&n4, maze_tree.data.get(&NodeType::Exit).unwrap());
    }

    #[test]
    fn side_upwards_path() {
        #[rustfmt::skip]
        let img = maze_image!([
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0]
        ]);

        let maze_tree = Maze::from_image(&img).unwrap();

        let mut n1 = Node::start(1, 0);
        let mut n2 = Node::at(3, 1);
        let mut n3 = Node::at(1, 2);
        let mut n4 = Node::at(3, 2);
        let mut n5 = Node::exit(1, 4);

        let mut map = FxHashMap::default();

        n1.children.insert(Direction::South, n3.point);
        n3.children.insert(Direction::North, n1.point);

        n3.children.insert(Direction::East, n4.point);
        n4.children.insert(Direction::West, n3.point);

        n4.children.insert(Direction::North, n2.point);
        n2.children.insert(Direction::South, n4.point);

        n3.children.insert(Direction::South, n5.point);
        n5.children.insert(Direction::North, n3.point);

        map.insert(NodeType::Start, n1);
        map.insert(NodeType::Path(n2.point), n2);
        map.insert(NodeType::Path(n3.point), n3);
        map.insert(NodeType::Path(n4.point), n4);
        map.insert(NodeType::Exit, n5);

        assert_eq!(&map, &maze_tree.data);
    }
}
