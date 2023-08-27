use std::{collections::HashMap, ops};

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub start: bool,
    pub end: bool,
    pub point: Point,
    pub children: HashMap<Direction, Point>,
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

    pub(crate) fn from_point(point: Point) -> Self {
        Self {
            start: false,
            end: false,
            point,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn at(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    North = 1,
    West = 2,
    South = 3,
    East = 4,
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Add<u32> for Direction {
    type Output = Direction;
    fn add(self, rhs: u32) -> Direction {
        let v = rhs % 4;
        match self {
            Direction::North => match v {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => unimplemented!(),
            },
            Direction::East => match v {
                0 => Direction::East,
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                _ => unimplemented!(),
            },
            Direction::South => match v {
                0 => Direction::South,
                1 => Direction::West,
                2 => Direction::North,
                3 => Direction::East,
                _ => unimplemented!(),
            },
            Direction::West => match v {
                0 => Direction::West,
                1 => Direction::North,
                2 => Direction::East,
                3 => Direction::South,
                _ => unimplemented!(),
            },
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Sub<u32> for Direction {
    type Output = Direction;
    fn sub(self, rhs: u32) -> Direction {
        let v = rhs % 4;
        match self {
            Direction::North => match v {
                0 => Direction::North,
                1 => Direction::West,
                2 => Direction::South,
                3 => Direction::East,
                _ => unimplemented!(),
            },
            Direction::East => match v {
                0 => Direction::East,
                1 => Direction::North,
                2 => Direction::West,
                3 => Direction::South,
                _ => unimplemented!(),
            },
            Direction::South => match v {
                0 => Direction::South,
                1 => Direction::East,
                2 => Direction::North,
                3 => Direction::West,
                _ => unimplemented!(),
            },
            Direction::West => match v {
                0 => Direction::West,
                1 => Direction::South,
                2 => Direction::East,
                3 => Direction::North,
                _ => unimplemented!(),
            },
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum NodeType {
    Start,
    Path(Point),
    Exit,
}
