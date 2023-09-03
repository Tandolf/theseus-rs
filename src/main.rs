use std::time::Instant;

use algorithms::Solver;
use image::{open, Rgb, RgbImage};
use maze::Maze;
use node::Point;

use crate::algorithms::left_turn::LeftTurn;

mod algorithms;
mod maze;
mod node;
mod utils;

// const TINY: &str = "mazes/tiny5x5.bmp";
// const MEDIUM: &str = "mazes/maze7x7_1.bmp";
// const LARGE: &str = "mazes/maze13x13_1.bmp";
const INSANE: &str = "mazes/maze99x99_1.bmp";
// const INSANE_10K: &str = "mazes/perfect10k.png";
// const INSANE_15K: &str = "mazes/perfect15k.png";

fn main() {
    let start = Instant::now();
    let mut image: RgbImage = open(INSANE).unwrap().into_rgb8();

    let maze = Maze::from_image(&image);
    let duration = start.elapsed();

    let maze = maze.unwrap();
    let mut solution = LeftTurn::solve(&maze).unwrap();
    // let mut solution = Dijkstra::solve(&maze).unwrap();

    println!("Number of nodes loaded: {}", maze.data.len());
    println!("Loading maze: {INSANE} took: {:?}", duration);

    let mut last = solution.path.pop_front().unwrap();
    for n in solution.path {
        let line = line(&last.point, &n.point);
        for point in line {
            image.put_pixel(point.x, point.y, Rgb([255, 0, 0]));
        }
        last = n;
    }

    image.save("solution.png").unwrap();
}

fn line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut line = Vec::new();

    let mut current = *p1;
    line.push(*p1);
    loop {
        if current == *p2 {
            break;
        }
        match p1.x.cmp(&p2.x) {
            std::cmp::Ordering::Less => {
                let n = Point::at(current.x + 1, current.y);
                line.push(n);
                current = n;
                continue;
            }
            std::cmp::Ordering::Equal if p1.y < p2.y => {
                let n = Point::at(current.x, current.y + 1);
                line.push(n);
                current = n;
                continue;
            }
            std::cmp::Ordering::Equal if p1.y > p2.y => {
                let n = Point::at(current.x, current.y - 1);
                line.push(n);
                current = n;
                continue;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                let n = Point::at(current.x - 1, current.y);
                line.push(n);
                current = n;
                continue;
            }
        }
    }
    line
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn east_line() {
        assert_eq!(
            line(&Point::at(0, 1), &Point::at(0, 3)),
            vec![Point::at(0, 1), Point::at(0, 2), Point::at(0, 3)],
        )
    }

    #[test]
    pub fn west_line() {
        assert_eq!(
            line(&Point::at(0, 3), &Point::at(0, 1)),
            vec![Point::at(0, 3), Point::at(0, 2), Point::at(0, 1)],
        )
    }

    #[test]
    pub fn south_line() {
        assert_eq!(
            line(&Point::at(1, 0), &Point::at(3, 0)),
            vec![Point::at(1, 0), Point::at(2, 0), Point::at(3, 0)],
        )
    }

    #[test]
    pub fn north_line() {
        assert_eq!(
            line(&Point::at(3, 0), &Point::at(1, 0)),
            vec![Point::at(3, 0), Point::at(2, 0), Point::at(1, 0)],
        )
    }
}
