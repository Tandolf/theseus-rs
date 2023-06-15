use std::{collections::HashMap, io::Error, iter};

use image::{Pixel, RgbImage};

use crate::node::{Direction, Node};
use crate::utils::{look_ahead, path_above, path_below, wall_above, wall_below};

pub struct Maze;

type Nodes = HashMap<(u32, u32), Node>;

pub const WALL: [u8; 3] = [0, 0, 0];
pub const PATH: [u8; 3] = [255, 255, 255];

// X = columns
// y = rows

fn get_entrance(image: &RgbImage, nodes: &mut Nodes, top_nodes: &mut [Option<(u32, u32)>]) {
    for (x, y, pixel) in image.enumerate_pixels() {
        if let [255, 255, 255] = pixel.channels() {
            let node = Node::at(x, y);
            nodes.insert((x, y), node);
            top_nodes[x as usize] = Some((x, y));
            break;
        }
    }
}

fn get_exit<'a>(image: &RgbImage, nodes: &'a mut Nodes, top_nodes: &'a [Option<(u32, u32)>]) {
    let last_row = image.height() - 1;
    for (x, pixel) in image.rows().last().unwrap().enumerate() {
        if pixel.channels() == PATH {
            let above = top_nodes[x].unwrap();
            let current = (x as u32, last_row);

            let mut exit_node = Node::at(current.0, current.1);
            exit_node.children.insert(Direction::North, above);
            nodes.insert(current, exit_node);

            let node_above = nodes.get_mut(&above).unwrap();
            node_above.children.insert(Direction::South, current);
            break;
        }
    }
}

impl Maze {
    pub(crate) fn from_image(image: RgbImage) -> Result<Nodes, Error> {
        let mut nodes = HashMap::new();

        let width = &image.width() - 1;
        let mut top_nodes: Vec<Option<(u32, u32)>> =
            iter::repeat_with(|| None).take(width as usize).collect();
        get_entrance(&image, &mut nodes, &mut top_nodes);

        let (width, height) = image.dimensions();
        if height > 2 {
            for y in 1..height - 1 {
                let mut current = false;
                let mut next = look_ahead(0, y, &image);

                let mut left_node = None;

                for x in 1..width - 1 {
                    let prev = current;
                    current = next;
                    next = look_ahead(x, y, &image);
                    println!(
                        "Point: {:?}, west: {}, ground: {}, east: {}, north: {}, south: {}",
                        &(x, y),
                        prev,
                        current,
                        next,
                        path_above(x, y, &image),
                        path_below(x, y, &image)
                    );

                    let mut n = None;

                    if !current {
                        continue;
                    }

                    if prev {
                        if next {
                            // PATH PATH PATH
                            // only create node if path above or below
                            // check above or below
                            if path_above(x, y, &image) || path_below(x, y, &image) {
                                let mut node = Node::at(x, y);
                                node.children.insert(Direction::West, left_node.unwrap());

                                nodes
                                    .get_mut(&left_node.unwrap())
                                    .unwrap()
                                    .children
                                    .insert(Direction::East, (x, y));

                                nodes.insert((x, y), node);
                                left_node = Some((x, y));
                                n = left_node;
                            }
                        } else {
                            // PATH PATH WALL
                            // Create Node at end of corridor
                            let mut node = Node::at(x, y);
                            node.children.insert(Direction::West, left_node.unwrap());

                            nodes
                                .get_mut(&left_node.unwrap())
                                .unwrap()
                                .children
                                .insert(Direction::East, (x, y));

                            nodes.insert((x, y), node);
                            left_node = None;
                            n = Some((x, y));
                        }
                    } else if next {
                        // WALL PATH PATH
                        // Create path at start of corridor
                        let node = Node::at(x, y);
                        nodes.insert((x, y), node);
                        left_node = Some((x, y));
                        n = Some((x, y));
                    } else {
                        // WALL PATH WALL
                        // Only create if in a dead end
                        if wall_above(x, y, &image) && wall_below(x, y, &image) {
                            let node = Node::at(x, y);
                            nodes.insert((x, y), node);
                            n = Some((x, y));
                        }
                    }

                    // If we have a node stored we can check if it needs to be connected upwards.
                    if let Some(current_n) = n {
                        // If path above, then there must be a Node to connect to above
                        if path_above(x, y, &image) {
                            let top_node_point = top_nodes[x as usize].unwrap();
                            nodes
                                .get_mut(&top_node_point)
                                .unwrap()
                                .children
                                .insert(Direction::South, (x, y));

                            nodes
                                .get_mut(&current_n)
                                .unwrap()
                                .children
                                .insert(Direction::North, top_node_point);
                        }

                        // If clear below, then this will probably be connectable, so place it in the top row
                        if path_below(x, y, &image) {
                            top_nodes[x as usize] = Some(current_n);
                        } else {
                            top_nodes[x as usize] = None;
                        }
                    }
                }
            }
        }

        if height > 1 {
            get_exit(&image, &mut nodes, &top_nodes);
        }

        // dbg!(&nodes);

        Ok(nodes)
    }
}

#[cfg(test)]
mod test {

    use image::Rgb;

    use super::*;

    const WALL: Rgb<u8> = Rgb([0, 0, 0]);
    const PATH: Rgb<u8> = Rgb([255, 255, 255]);

    // first maze row
    // xox
    #[test]
    fn entrance() {
        let mut img = RgbImage::new(3, 1);
        img.put_pixel(0, 0, WALL);
        img.put_pixel(1, 0, PATH);
        img.put_pixel(2, 0, WALL);

        let maze_tree = Maze::from_image(img).unwrap();

        let node = &maze_tree[&(1, 0)];
        assert_eq!(&Node::at(1, 0), node)
    }

    // two maze rows, second only walls
    // xox
    // xxx
    #[test]
    fn only_wall() {
        let mut img = RgbImage::new(3, 2);

        // first row
        img.put_pixel(0, 0, WALL);
        img.put_pixel(1, 0, PATH);
        img.put_pixel(2, 0, WALL);

        // second row
        img.put_pixel(0, 1, WALL);
        img.put_pixel(1, 1, WALL);
        img.put_pixel(2, 1, WALL);

        let maze_tree = Maze::from_image(img).unwrap();

        assert_eq!(&Node::at(1, 0), &maze_tree[&(1, 0)]);
        assert_eq!(maze_tree.get(&(1, 1)), None);
    }

    #[test]
    fn is_wall() {
        let mut img = RgbImage::new(2, 1);
        // first row
        img.put_pixel(0, 0, WALL);
        img.put_pixel(1, 0, PATH);

        let a1 = look_ahead(0, 0, &img);
        let a2 = look_ahead(1, 0, &img);
        dbg!(a1);
        assert!(!a1);
        assert!(a2);
    }

    // two maze rows, one middle path
    // xox
    // xox
    // xox
    #[test]
    fn first_corridor() {
        let mut img = RgbImage::new(3, 2);

        // first row
        img.put_pixel(0, 0, WALL);
        img.put_pixel(1, 0, PATH);
        img.put_pixel(2, 0, WALL);
        // second row
        img.put_pixel(0, 1, WALL);
        img.put_pixel(1, 1, PATH);
        img.put_pixel(2, 1, WALL);

        // third row
        img.put_pixel(0, 1, WALL);
        img.put_pixel(1, 1, PATH);
        img.put_pixel(2, 1, WALL);

        let maze_tree = Maze::from_image(img).unwrap();

        let mut n1 = Node::at(1, 0);
        n1.children.insert(Direction::South, (1, 1));

        let mut n2 = Node::at(1, 1);
        n2.children.insert(Direction::North, (1, 0));

        dbg!(&maze_tree);
        assert_eq!(&n1, &maze_tree[&(1, 0)]);
        assert_eq!(&n2, &maze_tree[&(1, 1)]);
    }
}
