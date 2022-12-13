use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

use image::{Rgb, RgbImage};

use crate::parse::{char_to_num, map_number};

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

#[derive(Eq, Clone)]
pub struct Point {
    x: u32,
    y: u32,
    height: u8,
    point_type: PointType,
}

impl Point {
    pub fn get_x(&self) -> u32 {
        self.x
    }
    pub fn get_y(&self) -> u32 {
        self.y
    }
    pub fn get_height(&self) -> u8 {
        self.height
    }
    pub fn get_point_type(&self) -> PointType {
        self.point_type.clone()
    }
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

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pt = match self.point_type {
            PointType::Regular => "",
            PointType::Start => "S",
            PointType::End => "E",
        };
        write!(f, "P{pt}({}, {})={}", self.x, self.y, self.height)
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
    parents: HashMap<Point, Option<Point>>,
    initial_node: Point,
    end_node: Point,
    tenative_distance_values: RefCell<HashMap<Point, f32>>,
    unvisited_nodes: HashSet<Point>,
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
        let unvisited_nodes = HashSet::from_iter(points.iter().map(|p| p.clone()));

        let parents = HashMap::from_iter(points.iter().map(|p| (p.clone(), None)));
        

        Self {
            points: HashSet::from_iter(points),
            parents,
            initial_node,
            end_node,
            tenative_distance_values: RefCell::new(tenative_distance_values),
            unvisited_nodes,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Point> {
        if x >= self.get_x_size() || y >= self.get_y_size() {
            return None;
        }
        self.points.get(&Point::coord(x, y))
    }

    pub fn iter_points(&self) -> std::collections::hash_set::Iter<Point> {
        self.points.iter()
    }

    pub fn get_tenative_distances(&self) -> HashMap<Point, f32> {
        HashMap::from_iter(
            self.tenative_distance_values
                .borrow()
                .iter()
                .map(|(p, num)| (p.clone(), num.clone())),
        )
    }

    pub fn get_tenative_distance(&self, point: &Point) -> Option<f32> {
        let distances = self.tenative_distance_values.borrow();
        match distances.get(point) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn get_unvisited_nodes(&self) -> HashSet<Point> {
        HashSet::from_iter(self.unvisited_nodes.iter().map(|p| p.clone()))
    }

    pub fn get_x_size(&self) -> u32 {
        self.points.iter().map(|point| point.x).max().unwrap() + 1
    }

    pub fn get_y_size(&self) -> u32 {
        self.points.iter().map(|point| point.y).max().unwrap() + 1
    }

    pub fn get_parent(&self, point: &Point) -> &Option<Point> {
        self.parents.get(point).expect(&format!("Point {:?} not found.", point))
    }

    pub fn find_neighbours_of_node(&self, node: &Point) -> [Option<&Point>; 4] {
        let xsize = self.get_x_size();
        let ysize = self.get_y_size();
        let left = if node.x == 0 {
            None
        } else {
            self.get(node.x - 1, node.y)
        };
        let right = if node.x < xsize - 1 {
            self.get(node.x + 1, node.y)
        } else {
            None
        };
        let top = if node.y == 0 {
            None
        } else {
            self.get(node.x, node.y - 1)
        };
        let bot = if node.y < ysize - 1 {
            self.get(node.x, node.y + 1)
        } else {
            None
        };

        [left, top, right, bot].map(|neigh| {
            // do not allow neighbours which are higher more than 1
            if let Some(n) = neigh {
                if n.height > node.height + 1 {
                    None
                } else {
                    Some(n)
                }
            } else {
                neigh
            }
        })
    }

    fn find_smallest_distance(&self) -> (Point, f32) {
        let distances = self.tenative_distance_values.borrow();
        let (p, v) = self.unvisited_nodes.iter().map(|p| {
            let val = distances.get(p).unwrap();
            (p, val)
        }).min_by_key(|(k, v)| **v as i32).expect("Min by key was not found");

        (p.clone(), v.clone())
    }

    pub fn process_shortest_distance(&mut self) {
        while !self.unvisited_nodes.is_empty() {
            let node_with_smallest_dist = self.find_smallest_distance();
            let (current_node, val) = node_with_smallest_dist;
            self.unvisited_nodes.remove(&current_node);
            if let PointType::End = current_node.point_type {
                println!("End found in {:?} with the distance of {}", current_node, self.get_tenative_distance(&current_node).unwrap());
                break;
            }
            let unexplored_neighbours: Vec<Point> = self.find_neighbours_of_node(&current_node)
                .iter()
                .filter(|neighbour| match neighbour {
                    None => false,
                    Some(point) => self.unvisited_nodes.contains(point),
                })
                .map(|n| n.unwrap().clone())
                .collect();
            // dbg!(&unexplored_neighbours);
            for neighbour in unexplored_neighbours {
                let distances = self.tenative_distance_values.borrow();
                let new_dist = 1.0 + distances.get(&current_node).expect(&format!("Could not find distance for {:?}", current_node));
                drop(distances);

                let mut distances_mut = self.tenative_distance_values.borrow_mut();
                let old_neigh_distance = distances_mut.get_mut(&neighbour).expect(&format!("Could not find distance for neighbour {:?}", neighbour));
                if new_dist < *old_neigh_distance {
                    *old_neigh_distance = new_dist;
                    self.parents.insert(neighbour, Some(current_node.clone()));
                }
            }
        }
    }

    pub fn find_path(&self) -> Vec<Point> {
        let mut result = Vec::new();
        let mut current_node = &self.end_node;
        while let Some(parent) = self.get_parent(current_node) {
            result.push(parent.clone());
            current_node = parent;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseFloatError;

    use crate::parse::parse_file;

    use super::{Point, PointType};

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

    #[test]
    fn test_find_closest_neighbours() {
        let m = parse_file("input.test");
        let neighbours = m.find_neighbours_of_node(&Point::coord(0, 0));
        assert_eq!(
            neighbours,
            [
                None,
                None,
                Some(&Point::coord(1, 0)),
                Some(&Point::coord(0, 1))
            ]
        );
        let r_neighbours = m.find_neighbours_of_node(&Point::from_map(3, 1, 'r'));
        assert_eq!(
            r_neighbours,
            [
                Some(&Point::coord(2, 1)),
                Some(&Point::coord(3, 0)),
                None,
                Some(&Point::coord(3, 2)),
            ]
        );
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
