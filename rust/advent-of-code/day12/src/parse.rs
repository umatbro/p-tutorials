use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::mountains::{MountainMap, Point, PointType};

pub fn char_to_num(ch: char) -> u8 {
    // Calculate the integer value for the character 'ch' by subtracting
    // the character 'a' and adding 1
    let value = ch as u8 - 'a' as u8 + 1;
    value
}

pub fn parse_file(file_name: &str) -> MountainMap {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for (y, l) in reader.lines().enumerate() {
        let line = l.unwrap();
        for (x, c) in line.chars().enumerate() {
            let point = Point::from_map(x as u32, y as u32, c);
            result.push(point);
        }
    }
    MountainMap::from_points(result)
}
