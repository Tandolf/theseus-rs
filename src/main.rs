use std::time::Instant;

use algorithms::Solver;
use image::{open, RgbImage};
use maze::Maze;

use crate::{algorithms::left_turn::LeftTurn, node::Direction};

mod algorithms;
mod maze;
mod node;
mod utils;

const TINY: &str = "mazes/tiny5x5.bmp";
// const MEDIUM: &str = "mazes/maze7x7_1.bmp";
// const LARGE: &str = "mazes/maze13x13_1.bmp";
// const INSANE: &str = "mazes/maze99x99_1.bmp";
// const INSANE_10K: &str = "mazes/perfect10k.png";
// const INSANE_15K: &str = "mazes/perfect15k.png";

fn main() {
    let start = Instant::now();
    let image: RgbImage = open(TINY).unwrap().into_rgb8();

    let maze = Maze::from_image(&image);
    let duration = start.elapsed();

    let maze = maze.unwrap();
    let _solution = LeftTurn::solve(&maze);

    println!("Number of nodes loaded: {}", maze.len());
    println!("Loading maze: {TINY} took: {:?}", duration);
}
