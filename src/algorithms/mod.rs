use std::collections::VecDeque;

use crate::maze::Nodes;
use crate::node::Node;

pub mod left_turn;

pub trait Solver {
    fn solve(maze: &Nodes) -> Option<Solution>;
}

pub struct Solution {
    count: u32,
    path: VecDeque<Node>,
    length: u32,
}

impl Solution {
    pub(crate) fn new(count: u32, path: VecDeque<Node>) -> Self {
        Self {
            count,
            path,
            length: 0,
        }
    }
}
