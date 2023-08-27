pub struct Dijkstra;

use crate::node::{
    NodeType::{self, Exit, Start},
    Point,
};
use std::collections::BinaryHeap;

use crate::{
    algorithms::{Solution, Solver},
    maze::Maze,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    pub key: u32,
    pub value: Point,
}

impl Node {
    pub fn new(key: u32, value: Point) -> Self {
        Node { key, value }
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
        let start_index = (start_point.x * width) + start_point.y;
        let mut distances: Vec<u32> = (0..total).map(|_| u32::MAX).collect();
        distances[start_index as usize] = 0;

        let mut unvisited = BinaryHeap::new();

        let start_node = Node::new(0, start_point);

        unvisited.push(&start_node);

        while !unvisited.is_empty() {
            let n = unvisited.pop().unwrap();
            let parent_point = n.value;
            let parent_index = (parent_point.x * width) + parent_point.y;

            let node = maze.get(&NodeType::Path(parent_point)).unwrap();

            // Iterate through each connecting child node
            for (_, child_point) in node.children.iter() {
                let child_index = (child_point.x * width) + child_point.y;

                // If we havnt checked the current node
                if !visited[child_index as usize] {
                    // Calculate the manhattan distance to the parent node
                    let abs_distance =
                        child_point.x - parent_point.x + child_point.y - parent_point.y;

                    // Get the current nodes current distance and add on the new distance
                    let new_distance = distances[parent_index as usize] + abs_distance;

                    // if this new distance is the shortest path
                    if new_distance < distances[child_index as usize] {
                        // foobar
                    }
                }
            }
        }

        None
    }
}
