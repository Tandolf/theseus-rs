use std::time::Instant;

use image::{open, RgbImage};
use maze::Maze;

mod maze;
mod node;
mod utils;

const TINY: &str = "mazes/tiny5x5.bmp";
// const MEDIUM: &str = "mazes/maze7x7_1.bmp";
// const LARGE: &str = "mazes/maze13x13_1.bmp";
// const INSANE: &str = "mazes/maze99x99_1.bmp";

fn main() {
    let image: RgbImage = open(TINY).unwrap().into_rgb8();

    let start = Instant::now();
    let maze = Maze::from_image(image);
    let duration = start.elapsed();

    println!("Number of nodes loaded: {}", maze.unwrap().len());
    println!("Loading maze: {TINY} took: {:?}", duration);
}
