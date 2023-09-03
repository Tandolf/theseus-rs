pub struct Dijkstra;

use crate::node::{
    NodeType::{self, Exit, Start},
    Point,
};
use std::collections::{BinaryHeap, VecDeque};

use crate::{
    algorithms::{Solution, Solver},
    maze::Maze,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    pub cost: u32,
    pub position: Point,
}

impl Node {
    pub fn new(cost: u32, position: Point) -> Self {
        Node { cost, position }
    }
}

impl Solver for Dijkstra {
    fn solve(maze: &Maze) -> Option<Solution> {
        let width = maze.width;
        let total = maze.height * maze.width;

        let visited: Vec<bool> = (0..total).map(|_| false).collect();

        let maze = &maze.data;
        if !maze.contains_key(&Start) {
            return None;
        }

        let start = &maze[&Start];
        let start_point = start.point;

        let end = &maze[&Exit];
        let end_point = end.point;

        let start_index = (start_point.y * width) + start_point.x;

        let mut distances: Vec<u32> = (0..total).map(|_| u32::MAX).collect();
        distances[start_index as usize] = 0;

        let mut unvisited = BinaryHeap::new();

        let state = Node::new(0, start_point);

        unvisited.push(state);

        while let Some(Node { cost, position }) = unvisited.pop() {
            let parent_index = (position.y * width) + position.x;

            let node = maze
                .get(&NodeType::Path(position))
                .unwrap_or(maze.get(&NodeType::Start).unwrap());

            // Iterate through each conneting child node
            for (_, child_point) in node.children.iter() {
                let child_index = (child_point.y * width) + child_point.x;

                // If we havnt checked the current no;
                if !visited[child_index as usize] {
                    // Calculate the manhattan distance to the start node
                    dbg!(child_point);
                    dbg!(position);
                    let abs_distance = child_point.y.saturating_sub(position.y)
                        + child_point.x.saturating_sub(position.x);

                    dbg!(parent_index);
                    dbg!(distances[parent_index as usize]);
                    // Get the current nodes current distance and add on the new distance
                    let new_distance = distances[parent_index as usize] + abs_distance;

                    // if this new distance is the shortest path
                    if new_distance < distances[child_index as usize] {
                        let node = Node::new(new_distance, *child_point);
                        unvisited.push(node);
                        distances[child_index as usize] = new_distance;
                    }
                }
            }
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

        Some(Solution {
            count: 0,
            length: 0,
            path: VecDeque::new(),
        })
    }
}
