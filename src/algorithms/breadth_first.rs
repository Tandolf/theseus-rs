use super::{Solution, Solver};
use crate::{
    maze::Maze,
    node::{
        Node,
        NodeType::{self, Exit, Start},
    },
};
use std::collections::VecDeque;

pub struct BreadthFirst;

impl Solver for BreadthFirst {
    fn solve(maze: &crate::maze::Maze) -> Option<Solution> {
        let width = maze.width;
        let total = maze.height * maze.width;

        let start = &maze.data[&Start];
        let end = &maze.data[&Exit];

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited: Vec<bool> = (0..total).map(|_| false).collect();
        let mut reverse_path: Vec<Option<&Node>> = (0..total).map(|_| None).collect();

        let mut decisions = 0;

        let start_index = (start.point.y * width) + start.point.x;
        visited[start_index as usize] = true;

        while let Some(current) = queue.pop_front() {
            decisions += 1;
            if current.point == end.point {
                break;
            }

            for (_, next_point) in current.children.iter() {
                let n_index = (next_point.y * width) + next_point.x;

                if !visited[n_index as usize] {
                    let node = maze
                        .data
                        .get(&NodeType::Path(*next_point))
                        .or(maze.data.get(&NodeType::Exit))
                        .unwrap();
                    queue.push_back(node);
                    visited[n_index as usize] = true;
                    reverse_path[n_index as usize] = Some(current);
                }
            }
        }
        let mut current = Some(&end);
        let mut solution = VecDeque::new();

        while current.is_some() {
            let node = current.unwrap();
            let node = get_node(node, maze).unwrap();

            solution.push_back(node);

            let index = (node.point.y * width) + node.point.x;
            current = reverse_path.get(index as usize).unwrap().as_ref();
        }

        Some(Solution::new(decisions, solution))
    }
}

fn get_node<'a>(node: &'a Node, maze: &'a Maze) -> Option<&'a Node> {
    maze.data.get(&NodeType::Path(node.point)).or_else(|| {
        if node.start {
            maze.data.get(&NodeType::Start)
        } else {
            maze.data.get(&NodeType::Exit)
        }
    })
}
