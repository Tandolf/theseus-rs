pub struct Dijkstra;

use crate::node::{
    Node,
    NodeType::{self, Exit, Start},
    Point,
};
use std::collections::{BinaryHeap, VecDeque};

use crate::{
    algorithms::{Solution, Solver},
    maze::Maze,
};

#[derive(Debug, Eq, PartialEq)]
struct BinNode {
    pub cost: u32,
    pub position: Point,
}

impl BinNode {
    pub fn new(cost: u32, position: Point) -> Self {
        BinNode { cost, position }
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
        let width = maze.width;
        let total = maze.height * maze.width;

        let mut visited: Vec<bool> = (0..total).map(|_| false).collect();
        let mut reverse_path: Vec<Option<Node>> = (0..total).map(|_| None).collect();

        let maze = &maze.data;
        if !maze.contains_key(&Start) {
            return None;
        }

        let start = &maze[&Start];
        let end = &maze[&Exit];
        // let _end_point = end.point;

        let start_index = (start.point.y * width) + start.point.x;

        let mut distances: Vec<u32> = (0..total).map(|_| u32::MAX).collect();
        distances[start_index as usize] = 0;

        let mut unvisited = BinaryHeap::new();
        unvisited.push(BinNode::new(0, start.point));

        while let Some(BinNode { cost: _, position }) = unvisited.pop() {
            let c_index = (position.y * width) + position.x;

            let node = maze
                .get(&NodeType::Path(position))
                .unwrap_or(maze.get(&NodeType::Start).unwrap());

            // Iterate through each conneting child node
            for (_, next_point) in node.children.iter() {
                let n_index = (next_point.y * width) + next_point.x;

                // If we havnt checked the current no;
                if !visited[n_index as usize] {
                    // Calculate the manhattan distance to the start node
                    let abs_distance = get_dist(&position, next_point);

                    // Get the current nodes current distance and add on the new distance
                    let new_distance = distances[c_index as usize] + abs_distance;

                    // if this new distance is the shortest path
                    if new_distance < distances[n_index as usize] {
                        let vnode = BinNode::new(new_distance, *next_point);
                        unvisited.push(vnode);
                        distances[n_index as usize] = new_distance;
                        reverse_path.insert(n_index as usize, Some(node.clone()));
                    }
                }
            }
            visited[c_index as usize] = true;
        }

        for d in 0..total {
            if d % width == 0 {
                println!();
            }
            if distances[d as usize] == u32::MAX {
                print!("{n:>3}", n = "");
            } else {
                print!("{n:>3}", n = distances[d as usize]);
            }
            print!("  ");
        }

        let mut solution = VecDeque::new();
        let mut current = Some(end);

        dbg!(&reverse_path);

        while current.is_some() {
            let c = current.unwrap();
            let n = maze
                .get(&NodeType::Path(c.point))
                .unwrap_or(maze.get(&NodeType::Start).unwrap());
            solution.push_back(n);
            current = reverse_path
                .get(((c.point.y * width) + c.point.x) as usize)
                .unwrap()
                .as_ref();
        }

        dbg!(&solution);

        Some(Solution {
            count: 0,
            length: 0,
            path: solution,
        })
    }
}

fn get_dist(current: &Point, next: &Point) -> u32 {
    ((next.y as i32 - current.y as i32).abs() + (next.x as i32 - current.x as i32).abs()) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn man_dist() {
        let d = get_dist(&Point::at(3, 0), &Point::at(3, 1));
        assert_eq!(1, d);

        let d = get_dist(&Point::at(3, 1), &Point::at(1, 1));
        assert_eq!(2, d);
    }
}
