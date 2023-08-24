use std::collections::VecDeque;

use crate::algorithms::{Solution, Solver};
use crate::maze::Maze;
use crate::node::NodeType::{Exit, Start};
use crate::node::{Direction, NodeType, Point};

// Left turn algorithm that will find the shortest path by only constantly taking left turns.
pub struct LeftTurn;

impl Solver for LeftTurn {
    fn solve(maze: &Maze) -> Option<Solution> {
        let maze = &maze.data;
        if !maze.contains_key(&Start) {
            return None;
        }

        let start = &maze[&Start];
        let child = &start.children[&Direction::South];
        let current = &maze.get(&NodeType::Path(Point::at(child.x, child.y)));
        let mut current = if current.is_none() {
            return None;
        } else {
            current.unwrap()
        };

        let mut path = VecDeque::from([start]);
        let mut heading = Direction::South;
        let turn = 1;
        let start = &maze[&Start].point;
        let end = &maze[&Exit].point;

        let mut count = 1;

        let completed = loop {
            path.push_back(current);
            count += 1;
            let coords = &current.point;
            if coords == start || coords == end {
                if coords == end {
                    break true;
                }
                break false;
            }
            let n = &current.children;

            // if there is a node to the left add that as a path
            if let Some(point) = n.get(&(heading - turn)) {
                heading = heading - turn;
                let node = maze
                    .get(&NodeType::Path(*point))
                    .unwrap_or(maze.get(&NodeType::Exit).unwrap());
                current = node;
                continue;
            };

            // if there is a node under us, use that as path
            if let Some(point) = n.get(&heading) {
                let node = maze
                    .get(&NodeType::Path(*point))
                    .unwrap_or(maze.get(&NodeType::Exit).unwrap());
                current = node;
                continue;
            };

            // if there is a node to the right, use that as path
            if let Some(point) = n.get(&(heading + turn)) {
                heading = heading + turn;
                let node = maze
                    .get(&NodeType::Path(*point))
                    .unwrap_or(maze.get(&NodeType::Exit).unwrap());
                current = node;
                continue;
            };

            // last resort, turn back
            if let Some(point) = n.get(&(heading + (turn * 2))) {
                heading = heading + (turn * 2);
                let node = maze
                    .get(&NodeType::Path(*point))
                    .unwrap_or(maze.get(&NodeType::Exit).unwrap());
                current = node;
                continue;
            };

            break false;
        };

        if !completed {
            return None;
        }

        Some(Solution::new(count, path))
    }
}
