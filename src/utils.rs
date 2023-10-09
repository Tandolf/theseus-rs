use image::{Pixel, RgbImage};

use crate::{
    algorithms::Algorithm,
    maze::{Maze, PATHWAY, WALL},
    node::{Node, NodeType, Point},
    Cli,
};

pub fn look_ahead(x: u32, y: u32, image: &RgbImage) -> bool {
    is_path(x + 1, y, image)
}

pub fn path_above(x: u32, y: u32, image: &RgbImage) -> bool {
    is_path(x, y - 1, image)
}

pub fn path_below(x: u32, y: u32, image: &RgbImage) -> bool {
    is_path(x, y + 1, image)
}

pub fn is_path(x: u32, y: u32, image: &RgbImage) -> bool {
    image
        .get_pixel_checked(x, y)
        .is_some_and(|pix| pix.channels() == PATHWAY)
}

pub fn wall_above(x: u32, y: u32, image: &RgbImage) -> bool {
    is_wall(x, y - 1, image)
}

pub fn wall_below(x: u32, y: u32, image: &RgbImage) -> bool {
    is_wall(x, y + 1, image)
}

pub fn is_wall(x: u32, y: u32, image: &RgbImage) -> bool {
    image
        .get_pixel_checked(x, y)
        .is_some_and(|pix| pix.channels() == WALL)
}

pub fn get_algorithm(cli: &Cli) -> Algorithm {
    if cli.dijkstra {
        Algorithm::Dijkstra
    } else if cli.a_star {
        Algorithm::AStar
    } else if cli.left_turn {
        Algorithm::LeftTurn
    } else if cli.breadth_first {
        Algorithm::BreadthFirst
    } else if cli.depth_first {
        Algorithm::DepthFirst
    } else {
        Algorithm::None
    }
}

pub fn get_dist(current: &Point, next: &Point) -> u32 {
    ((next.y as i32 - current.y as i32).abs() + (next.x as i32 - current.x as i32).abs()) as u32
}

pub fn get_node<'a>(node: &'a Node, maze: &'a Maze) -> Option<&'a Node> {
    maze.data.get(&NodeType::Path(node.point)).or_else(|| {
        if node.start {
            maze.data.get(&NodeType::Start)
        } else {
            maze.data.get(&NodeType::Exit)
        }
    })
}
