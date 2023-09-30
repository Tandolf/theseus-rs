pub struct Dijkstra;

use crate::node::{
    Node,
    NodeType::{self, Exit, Start},
    Point,
};
use std::{collections::{BinaryHeap, VecDeque}, cell::Cell};

use crate::{
    algorithms::{Solution, Solver},
    maze::Maze,
};

#[derive(Debug, Eq, PartialEq)]
struct BinNode {
    pub cost: u32,
    pub position: Point,
    is_valid: Cell<bool>,
}

impl BinNode {
    pub fn new(cost: u32, position: Point) -> Self {
        BinNode { cost, position, is_valid: Cell::new(true)}
    }
}

impl Ord for BinNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for BinNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solver for Dijkstra {
    fn solve(maze: &Maze) -> Option<Solution> {

        let mut decisions = 0;

        let width = maze.width;
        let total = maze.height * maze.width;

        let mut visited: Vec<bool> = (0..total).map(|_| false).collect();
        let mut reverse_path: Vec<Option<&Node>> = (0..total).map(|_| None).collect();

        if !maze.data.contains_key(&Start) {
            return None;
        }

        let start = &maze.data[&Start];
        let end = &maze.data[&Exit];

        let start_index = (start.point.y * width) + start.point.x;

        let mut distances: Vec<u32> = (0..total).map(|_| u32::MAX).collect();
        distances[start_index as usize] = 0;

        let mut unvisited = BinaryHeap::new();
        unvisited.push(BinNode::new(0, start.point));

        while let Some(BinNode { cost: _, position, is_valid }) = unvisited.pop() {

            decisions += 1;

            if position == end.point {
                break;
            }

            if !is_valid.take() {
                continue;
            }

            let node = maze.data
                .get(&NodeType::Path(position))
                .or(maze.data.get(&NodeType::Start))
                .unwrap();

            let c_index = (position.y * width) + position.x;

            // Iterate through each conneting child node
            for (_, next_point) in node.children.iter() {
                let n_index = (next_point.y * width) + next_point.x;

                // If we havnt checked the current no;
                if !visited[n_index as usize] {
                    // Calculate the manhattan distance to the start node
                    let abs_distance = get_dist(&position, next_point);

                    // Get the current nodes current distance and add on the new distance
                    let mut new_distance = distances[c_index as usize] + abs_distance;

                    // if this new distance is the shortest path
                    if new_distance < distances[n_index as usize] {
                        reverse_path[n_index as usize] = Some(node);
                    } else {
                        let node = unvisited.iter().find(|n| &n.position == next_point).unwrap();
                        node.is_valid.set(false);
                        new_distance += node.cost;
                    }
                    let node = BinNode::new(new_distance, *next_point);
                    unvisited.push(node);
                    distances[n_index as usize] = new_distance;
                }
            }

            visited[c_index as usize] = true;
        }

        let mut current = Some(&end);
        let mut solution = VecDeque::new();

        while current.is_some() {
            let node = current.unwrap();
            let node = get_node(node, maze).unwrap();

            solution.push_back(node);

            let index = (node.point.y * width) + node.point.x;
            current = reverse_path
                .get(index as usize)
                .unwrap()
                .as_ref();
        }

        Some(Solution::new(decisions, solution))
    }
}

fn get_dist(current: &Point, next: &Point) -> u32 {
    ((next.y as i32 - current.y as i32).abs() + (next.x as i32 - current.x as i32).abs()) as u32
}

fn get_node<'a>(node: &'a Node, maze: &'a Maze) -> Option<&'a Node> {
    maze.data.get(&NodeType::Path(node.point))
        .or_else(|| {
            if node.start {
                maze.data.get(&NodeType::Start)
            } else {
                maze.data.get(&NodeType::Exit)
            }
        })
}

#[cfg(test)]
mod test {

    use super::*;
    use image::Rgb;
    use image::RgbImage;
    use pretty_assertions::assert_eq;

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

    fn create_path<'a>(coords: &'a [(u32, u32)], maze: &'a Maze) -> VecDeque<&'a Node> {
        let mut path = VecDeque::new();
        if let Some(node) = maze.data.get(&NodeType::Start) { path.push_front(node); }

        for coord in coords.iter() {
            path.push_front(maze.data.get(&NodeType::Path(Point::at(coord.0, coord.1))).unwrap());
        }
            
        path.push_front(maze.data.get(&NodeType::Exit).unwrap());
        path 
    }

    #[test]
    fn man_dist() {
        let d = get_dist(&Point::at(3, 0), &Point::at(3, 1));
        assert_eq!(1, d);

        let d = get_dist(&Point::at(3, 1), &Point::at(1, 1));
        assert_eq!(2, d);
    }

    #[test]
    fn simple_maze() {
        let img = maze_image!([
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0]
        ]);

        let maze = Maze::from_image(&img).unwrap();
        let solution = Dijkstra::solve(&maze).unwrap();
        let path = create_path(&[(1,2)], &maze);
        assert_eq!(path, solution.path)
    }

    #[test]
    fn maze_with_loop() {
        let img = maze_image!([
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0]
        ]);

        let maze = Maze::from_image(&img).unwrap();
        let solution = Dijkstra::solve(&maze).unwrap();
        let path = create_path(&[(1,1), (1,3)], &maze);
        assert_eq!(path, solution.path)
    }

    #[test]
    fn maze_with_loop_opposite_exit() {
        let img = maze_image!([
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0]
        ]);

        let maze = Maze::from_image(&img).unwrap();
        let solution = Dijkstra::solve(&maze).unwrap();
        let path = create_path(&[(1,1), (1,3), (3, 3)], &maze);
        assert_eq!(path, solution.path)
    }

    #[test]
    fn maze_walking_away() {
        let img = maze_image!([
            [0, 0, 0, 0, 0, 1, 0],
            [0, 1, 1, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0, 0, 0]
        ]);

        let maze = Maze::from_image(&img).unwrap();
        let solution = Dijkstra::solve(&maze).unwrap();
        let path = create_path(&[
            (5,3), 
            (3,3), 
            (3,1), 
            (1,1)
        ], &maze);
        assert_eq!(path, solution.path)
    }

    #[test]
    fn maze_walking_west() {
        let img = maze_image!([
            [0, 0, 0, 1, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 0],
            [0, 1, 0, 0, 0, 1, 0],
            [0, 1, 1, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0]
        ]);

        let maze = Maze::from_image(&img).unwrap();
        let solution = Dijkstra::solve(&maze).unwrap();
        let path = create_path(&[
            (3,1), 
            (1,1),
            (1,3),
            (2,3)
        ], &maze);
        assert_eq!(path, solution.path)
    }

    #[test]
    fn maze_walking_east() {
        let img = maze_image!([
            [0, 0, 0, 1, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 0],
            [0, 1, 0, 0, 0, 1, 0],
            [0, 1, 1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1, 0, 0]
        ]);

        let maze = Maze::from_image(&img).unwrap();
        let solution = Dijkstra::solve(&maze).unwrap();
        let path = create_path(&[
            (3,1), 
            (5,1),
            (5,3),
            (4,3)
        ], &maze);
        assert_eq!(path, solution.path)
    }
}
