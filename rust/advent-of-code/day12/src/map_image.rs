use image::{self, ImageBuffer};
use image::{Rgb, RgbImage};

use crate::mountains::{MountainMap, PointType};
use crate::parse::map_number;

pub enum MapType {
    Height,
    DistanceFromStart,
    Visited,
}

pub struct MapImageOptions {
    pub file_name: String,
    pub map_type: MapType,
    pub draw_path: bool,
}


pub fn save_image(map: &MountainMap, opts: MapImageOptions) -> Result<(), image::ImageError> {
    let func = match opts.map_type {
        MapType::Height => save_height_image,
        MapType::DistanceFromStart => save_distance_image,
        MapType::Visited => save_visited_image,
    };
    func(map, opts)
}

fn save_height_image(map: &MountainMap, opts: MapImageOptions) -> Result<(), image::ImageError> {
    let mut img = RgbImage::new(map.get_x_size(), map.get_y_size());
    for p in map.iter_points() {
        let color_val = match p.get_point_type() {
            PointType::Regular => [map_number(p.get_height()); 3],
            _ => [0, 0, 255],
        };
        img.put_pixel(p.get_x(), p.get_y(), Rgb(color_val));
    }
    if opts.draw_path {
        for point in map.find_path() {
            img.put_pixel(point.get_x(), point.get_y(), Rgb([0, 255, 0]))
        }
    }
    img.save(opts.file_name)
}

fn save_distance_image(map: &MountainMap, opts: MapImageOptions) -> Result<(), image::ImageError> {
    let mut img = RgbImage::new(map.get_x_size(), map.get_y_size());
    use std::cmp::min;

    for (p, dist) in map.get_tenative_distances() {
        let color_val = if dist.is_finite() {
            [min(dist as i32, 255) as u8; 3]
        } else {
            [255, 0, 0]
        };
        img.put_pixel(p.get_x(), p.get_y(), Rgb(color_val));
    }

    if opts.draw_path {
        for point in map.find_path() {
            img.put_pixel(point.get_x(), point.get_y(), Rgb([0, 255, 0]))
        }
    }

    img.save(opts.file_name)
}

fn save_visited_image(map: &MountainMap, opts: MapImageOptions) -> Result<(), image::ImageError> {
    let mut img = RgbImage::new(map.get_x_size(), map.get_y_size());
    for y in 0..map.get_y_size() {
        for x in 0..map.get_x_size() {
            img.put_pixel(x, y, Rgb([0, 0, 255]));
        }
    };
    for p in map.get_unvisited_nodes() {
        img.put_pixel(p.get_x(), p.get_y(), Rgb([255, 0, 0]));
    }
    if opts.draw_path {
        for point in map.find_path() {
            img.put_pixel(point.get_x(), point.get_y(), Rgb([0, 255, 0]))
        }
    }
    img.save(opts.file_name)
}
