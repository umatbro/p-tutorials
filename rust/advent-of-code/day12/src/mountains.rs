use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use image::{Rgb, RgbImage};

use crate::parse::char_to_num;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PointType {
    Start,
    End,
    Regular,
}

impl FromStr for PointType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PointType::*;

        let c = s.chars().nth(0).unwrap();
        match c {
            'S' => Ok(Start),
            'E' => Ok(End),
            _ => Ok(Regular),
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Point {
    x: u32,
    y: u32,
    height: u8,
    point_type: PointType,
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn from_map(x: u32, y: u32, height_char: char) -> Self {
        let point_type = height_char.to_string().parse::<PointType>().unwrap();
        let height = match point_type {
            PointType::End => 'z',
            PointType::Start => 'a',
            _ => height_char,
        };
        Self {
            x,
            y,
            height: char_to_num(height),
            point_type,
        }
    }

    pub fn coord(x: u32, y: u32) -> Self {
        Point {
            x,
            y,
            height: 0,
            point_type: PointType::Regular,
        }
    }
}

pub struct MountainMap {
    points: HashSet<Point>,
    initial_node: Point,
    end_node: Point,
    tenative_distance_values: HashMap<Point, f32>,
    visited_nodes: HashMap<Point, bool>,
    current_node: Point,
}

impl MountainMap {
    pub fn from_points(points: Vec<Point>) -> Self {
        let initial_node = points
            .iter()
            .filter(|p| match p.point_type {
                PointType::Start => true,
                _ => false,
            })
            .nth(0)
            .unwrap()
            .clone();
        let end_node = points
            .iter()
            .filter(|p| match p.point_type {
                PointType::End => true,
                _ => false,
            })
            .nth(0)
            .unwrap()
            .clone();
        let tenative_distance_values = HashMap::from_iter(points.iter().map(|point| {
            let val = match point.point_type {
                PointType::Start => 0.,
                _ => f32::INFINITY,
            };
            (point.clone(), val)
        }));
        let visited_nodes = HashMap::from_iter(points.iter().map(|point| (point.clone(), false)));
        let current_node = initial_node.clone();
        Self {
            points: HashSet::from_iter(points),
            initial_node,
            end_node,
            tenative_distance_values,
            visited_nodes,
            current_node,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Point> {
        self.points.get(&Point::coord(x, y))
    }

    pub fn get_x_size(&self) -> u32 {
        self.points.iter().map(|point| point.x).max().unwrap() + 1
    }

    pub fn get_y_size(&self) -> u32 {
        self.points.iter().map(|point| point.y).max().unwrap() + 1
    }

    pub fn find_neighbours_of_node(&self, node: &Point) -> [Option<&Point>; 4] {
        let left = self.get(node.x - 1, node.y);
        let right = self.get(node.x - 1, node.y);
        let top = self.get(node.x, node.y - 1);
        let bot = self.get(node.x, node.y + 1);

        [left, top, right, bot]
    }

    fn calculate_neighbour_distances(&mut self) {
        println!("Checking node {:?}", self.current_node);
        let neighbours = self.find_neighbours_of_node(&self.current_node);
        dbg!(&neighbours);
        for neighbour in neighbours {
            if let Some(n) = neighbour {
                let dist = self.tenative_distance_values.get_mut(n).unwrap();
            }
        }
    }

    pub fn save_map_image(&self, file_name: &str) -> Result<(), image::ImageError> {
        let mut img = RgbImage::new(self.get_x_size(), self.get_y_size());

        for p in self.points.iter() {
            let color_val = map_number(p.height);
            img.put_pixel(p.x, p.y, Rgb([color_val; 3]))
        }

        img.save(file_name)
    }
}

fn map_number(n: u8) -> u8 {
    if n < 1 || n > 26 {
        return 0;
    }

    255 - (n - 1) * 10
}

#[cfg(test)]
mod tests {
    use std::num::ParseFloatError;

    use crate::parse::parse_file;

    use super::PointType;

    #[test]
    fn test_example_size() {
        let m = parse_file("input.test");
        assert_eq!(m.get_x_size(), 8);
        assert_eq!(m.get_y_size(), 5);
        assert_eq!(m.initial_node.x, 0);
        assert_eq!(m.initial_node.y, 0);
        assert_eq!(m.end_node.x, 5);
        assert_eq!(m.end_node.y, 2);
    }

    #[test]
    fn test_access_points() {
        let m = parse_file("input.test");
        let point = m.get(0, 0).unwrap();
        assert_eq!(point.point_type, PointType::Start);
        assert_eq!(point.height, 1);
        let end = m.get(5, 2).unwrap();
        assert_eq!(end.point_type, PointType::End);
        assert_eq!(end.height, 26);

        let regular = m.get(1, 1).unwrap();
        assert_eq!(regular.point_type, PointType::Regular);
        assert_eq!(regular.height, 2);
    }
}
// struct Point {
//     x: i32,
//     y: i32,
//     height: u32,
//     prev: Option<Box<Point>>,
// }

// fn dijkstra(start: Point, end: Point) -> Vec<Point> {
//     // Create a priority queue to store the nodes that are yet to be visited,
//     // with the distance as the priority.
//     let mut queue = BinaryHeap::new();

//     // Set the distance of the starting point to 0 and add it to the queue.
//     start.distance = 0;
//     queue.push(start);

//     // Create a set to store the nodes that have already been visited.
//     let mut visited = HashSet::new();

//     // Loop until the queue is empty or the end point has been reached.
//     while let Some(point) = queue.pop() {
//         // If the end point has been reached, return the path.
//         if point == end {
//             return reconstruct_path(&point);
//         }

//         // Skip this point if it has already been visited.
//         if !visited.insert(point) {
//             continue;
//         }

//         // Get the coordinates of the current point.
//         let x = point.x;
//         let y = point.y;

//         // Add the adjacent points to the queue.
//         queue.push(Point { x: x - 1, y, distance: point.distance + 1, prev: Some(Box::new(point)) });
//         queue.push(Point { x: x + 1, y, distance: point.distance + 1, prev: Some(Box::new(point)) });
//         queue.push(Point { x, y: y - 1, distance: point.distance + 1, prev: Some(Box::new(point)) });

//     }

//     Vec::new()
// }
