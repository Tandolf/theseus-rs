use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub start: bool,
    pub end: bool,
    pub point: Point,
    pub children: HashMap<Direction, (u32, u32)>,
}

impl Node {
    pub(crate) fn at(x: u32, y: u32) -> Self {
        Self {
            start: false,
            end: false,
            point: Point::at(x, y),
            children: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    fn at(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    West,
    South,
    East,
}
