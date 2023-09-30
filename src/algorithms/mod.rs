use std::collections::VecDeque;

use crate::maze::Maze;
use crate::node::Node;

pub mod dijkstra;
pub mod a_star;
pub mod left_turn;

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
