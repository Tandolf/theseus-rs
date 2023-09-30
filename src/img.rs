use std::path::Path;

use image::{ImageBuffer, Rgb, open};

use crate::{algorithms::Solution, node::Point};

pub struct Image {
    pub image: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Image {
    pub fn open(path: &Path) -> Self {
        let image = open(path).unwrap().into_rgb8();
        Self {
            image
        }
    }

    pub fn apply_solution(&mut self, solution: &mut Solution) {
        let mut last = solution.path.pop_front().unwrap();
        for n in &solution.path {
            let line = line(&last.point, &n.point);
            for point in line {
               self.image.put_pixel(point.x, point.y, Rgb([255, 0, 0]));
            }
            last = n;
        }
    }

    pub fn save(&self, output: &std::path::Path) -> Result<(), image::ImageError> {
        self.image.save(output)
    }
}

pub (crate) fn line(p1: &Point, p2: &Point) -> Vec<Point> {
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
