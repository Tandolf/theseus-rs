use std::collections::VecDeque;
use std::fmt::Display;

use crate::maze::Maze;
use crate::node::Node;

pub mod a_star;
pub mod breadth_first;
pub mod depth_first;
pub mod dijkstra;
pub mod left_turn;

pub enum Algorithm {
    LeftTurn,
    Dijkstra,
    AStar,
    BreadthFirst,
    None,
    DepthFirst,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let algorithm = match self {
            Algorithm::LeftTurn => "Left Turn ◀️◀️◀️",
            Algorithm::Dijkstra => "Dijkstra 👴",
            Algorithm::AStar => "A🌟",
            Algorithm::BreadthFirst => "Breadth First 🍞",
            Algorithm::DepthFirst => "Depth First",
            _ => unimplemented!(),
        };
        write!(f, "{}", algorithm)
    }
}

pub trait Solver {
    fn solve(maze: &Maze) -> Option<Solution>;
}

#[derive(Debug)]
pub struct Solution<'a> {
    pub count: u32,
    pub path: VecDeque<&'a Node>,
    pub length: usize,
}

impl<'a> Solution<'a> {
    pub(crate) fn new(count: u32, path: VecDeque<&'a Node>) -> Self {
        let length = path.len();
        Self {
            count,
            path,
            length,
        }
    }
}
