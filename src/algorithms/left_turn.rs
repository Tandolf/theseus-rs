use std::collections::VecDeque;

use crate::algorithms::{Solution, Solver};
use crate::maze::Nodes;
use crate::node::NodeType::{Exit, Start};
use crate::node::{Direction, NodeType, Point};

// Left turn algorithm that will find the shortest path by only constantly taking left turns.
pub struct LeftTurn;

impl Solver for LeftTurn {
    fn solve(maze: &Nodes) -> Option<Solution> {
        if !maze.contains_key(&Start) {
            return None;
        }

        let start = &maze[&Start];
        let child = &start.children[&Direction::South];
        let current = &maze.get(&NodeType::Path(Point::at(child.x, child.y)));
        let _current = if current.is_none() {
            return None;
        } else {
            current.unwrap()
        };

        let _path = VecDeque::from([start]);
        let _heading = Direction::South;
        let _turn = 1;
        let _s_point = &maze[&Start].point;
        let _e_point = &maze[&Exit].point;

        let _count = 1;

        None
    }
}
