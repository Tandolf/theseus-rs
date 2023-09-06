use image::{Pixel, RgbImage};

use crate::maze::{PATHWAY, WALL};

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
