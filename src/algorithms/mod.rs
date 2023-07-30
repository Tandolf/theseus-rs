use std::collections::VecDeque;

use crate::maze::Nodes;
use crate::node::Node;

pub mod left_turn;

pub trait Solver {
    fn solve(maze: &Nodes) -> Option<Solution>;
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
