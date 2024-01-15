use std::{fmt::Debug, str::FromStr};
use crate::test_area::TestArea;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: f64, 
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, PartialEq)]
pub struct Velocity {
    pub x: i32, 
    pub y: i32,
    pub z: i32,
}

impl Velocity {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}


#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub original_input: String,
}

impl ParseError {
    pub fn new(original_input: &str, message: &str) -> Self {
        Self { message: message.to_string(), original_input: original_input.to_string() }
    }
}

#[derive(Debug)]
pub enum Intersection {
    None,
    InArea(Position),
    OutsideArea(Position),
    InPast,
}

pub struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl Hailstone {
    /// Get line parameters based on two points - current and next
    /// y = ax + b
    /// a = (y2 - y1) / (x2 - x1)
    /// b = y1 - a * x1
    fn get_parameters(&self) -> (f64, f64) {
        let point1 = &self.position;
        let point2 = Position::new(
            self.position.x + self.velocity.x as f64,
            self.position.y + self.velocity.y as f64,
            self.position.z + self.velocity.z as f64
        );
        let a = (point2.y - point1.y) / (point2.x - point1.x);
        let b = point1.y - a * point1.x;
        (a, b)
    }

    /// Check if intersection with other hailstone happens in the past.
    /// We consider that intersection happens in the past if the intersection point is
    /// behind the current position of any of the hailstones.
    fn check_if_intersection_in_past(&self, intersection_position: &Position) -> bool {
        let x_direction = intersection_position.x - self.position.x;
        let y_direction = intersection_position.y - self.position.y;
        let x_direction = x_direction.is_sign_positive();
        let y_direction = y_direction.is_sign_positive();
        let v_x_direction = self.velocity.x >= 0;
        let v_y_direction = self.velocity.y >= 0;

        if (x_direction == v_x_direction) && (y_direction == v_y_direction) {
            return false;
        }
        true
    }

    pub fn find_interesection(&self, other: &Self, test_area: &TestArea) -> Intersection {
        let (a1, b1) = self.get_parameters();
        let (a2, b2) = other.get_parameters();

        if a1 == a2 {
            return Intersection::None;
        }

        let x = (b2 - b1) / (a1 - a2);
        let y = a1 * x + b1;
        let position = Position::new(x, y, 0.0);

        if self.check_if_intersection_in_past(&position) || other.check_if_intersection_in_past(&position) {
            return Intersection::InPast;
        }

        match test_area.is_in_area(&position) {
            true => Intersection::InArea(position),
            false => Intersection::OutsideArea(position),
        }
    }
}

impl Debug for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} @ {}, {}, {}", 
            self.position.x, self.position.y, self.position.z,
            self.velocity.x, self.velocity.y, self.velocity.z)
    }
}

impl PartialEq for Hailstone {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl FromStr for Hailstone {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("@");
        let position = parts.next().ok_or(ParseError::new(s, "Missing position"))?;
        let velocity = parts.next().ok_or(ParseError::new(s, "Missing velocity"))?;

        let position = position.trim();
        let velocity = velocity.trim();

        let mut position_parts = position.split(",");
        let mut velocity_parts = velocity.split(",");

        let position_x = position_parts.next().ok_or(ParseError::new(s, "Missing position x"))?.trim().parse::<f64>().map_err(|_| ParseError::new(s, "Invalid position x"))?;
        let position_y = position_parts.next().ok_or(ParseError::new(s, "Missing position y"))?.trim().parse::<f64>().map_err(|_| ParseError::new(s, "Invalid position y"))?;
        let position_z = position_parts.next().ok_or(ParseError::new(s, "Missing position z"))?.trim().parse::<f64>().map_err(|_| ParseError::new(s, "Invalid position z"))?;

        let velocity_x = velocity_parts.next().ok_or(ParseError::new(s, "Missing velocity x"))?.trim().parse::<i32>().map_err(|_| ParseError::new(s, "Invalid velocity x"))?;
        let velocity_y = velocity_parts.next().ok_or(ParseError::new(s, "Missing velocity y"))?.trim().parse::<i32>().map_err(|_| ParseError::new(s, "Invalid velocity y"))?;
        let velocity_z = velocity_parts.next().ok_or(ParseError::new(s, "Missing velocity z"))?.trim().parse::<i32>().map_err(|_| ParseError::new(s, "Invalid velocity z"))?;

        Ok(Self { position: Position::new(position_x, position_y, position_z), velocity: Velocity::new(velocity_x, velocity_y, velocity_z) })
    }
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::test_area;

    use super::{Position, Velocity, Hailstone, Intersection};
    use std::str::FromStr;

    #[rstest]
    #[case("0, 0, 0 @ 0, 0, 0", Hailstone { position: Position::new(0.0, 0.0, 0.0), velocity: Velocity::new(0, 0, 0)})]
    #[case("20, 19, 15 @  1, -5, -3", Hailstone { position: Position::new(20.0, 19.0, 15.0), velocity: Velocity::new(1, -5, -3)})]
    #[case("18, 19, 22 @ -1, -1, -2", Hailstone { position: Position::new(18.0, 19.0, 22.0), velocity: Velocity::new(-1, -1, -2)})]
    #[case("71077865007748, 250908661982152, 153144417138652 @ 147, 18, 159", Hailstone { position: Position::new(71077865007748.0, 250908661982152.0, 153144417138652.0), velocity: Velocity::new(147, 18, 159)})]
    fn test_from_str(#[case] input: &str, #[case] expected: Hailstone) {
        assert_eq!(input.parse::<Hailstone>().unwrap(), expected);
    }

    #[rstest]
    #[case(
        Hailstone::from_str("19, 13, 30 @ -2,  1, -2").unwrap(),
        Hailstone::from_str("18, 19, 22 @ -1, -1, -2").unwrap(),
        Intersection::InArea(Position::new(14.333, 15.333, 0.0)),
    )]
    #[case(
        Hailstone::from_str("19, 13, 30 @ -2,  1, -2").unwrap(),
        Hailstone::from_str("20, 25, 34 @ -2, -2, -4").unwrap(),
        Intersection::InArea(Position::new(11.667, 16.667, 0.0)),
    )]
    #[case(
        Hailstone::from_str("19, 13, 30 @ -2, 1, -2").unwrap(),
        Hailstone::from_str("12, 31, 28 @ -1, -2, -1").unwrap(),
        Intersection::OutsideArea(Position::new(6.2, 19.4, 0.0)),
    )]
    #[case(
        Hailstone::from_str("19, 13, 30 @ -2, 1, -2").unwrap(),
        Hailstone::from_str("20, 19, 15 @ 1, -5, -3").unwrap(),
        Intersection::InPast,
    )]
    #[case(
        Hailstone::from_str("18, 19, 22 @ -1, -1, -2").unwrap(),
        Hailstone::from_str("20, 25, 34 @ -2, -2, -4").unwrap(),
        Intersection::None,
    )]
    #[case(
        Hailstone::from_str("18, 19, 22 @ -1, -1, -2").unwrap(),
        Hailstone::from_str("12, 31, 28 @ -1, -2, -1").unwrap(),
        Intersection::OutsideArea(Position::new(-6.0, -5.0, 0.0)),
    )]
    #[case(
        Hailstone::from_str("18, 19, 22 @ -1, -1, -2").unwrap(),
        Hailstone::from_str("20, 19, 15 @ 1, -5, -3").unwrap(),
        Intersection::InPast
    )]
    fn test_intersection(#[case] hailstone1: Hailstone, #[case] hailstone2: Hailstone, #[case] expected: Intersection) {
        let tolerance = 0.001;
        let test_area = test_area::TestArea::new(7, 27);
        let intersection = hailstone1.find_interesection(&hailstone2, &test_area);
        match expected {
            Intersection::None => assert!(matches!(intersection, Intersection::None), "Expected None, got {:?}", intersection),
            Intersection::InPast => assert!(matches!(intersection, Intersection::InPast), "Expected InPast, got {:?}", intersection),
            Intersection::InArea(_) => assert!(matches!(intersection, Intersection::InArea(_) | Intersection::OutsideArea(_)), "Expected InArea, got {:?}", intersection),
            Intersection::OutsideArea(_) => assert!(matches!(intersection, Intersection::InArea(_) | Intersection::OutsideArea(_)), "Expected OutsideArea, got {:?}", intersection),
        }

        if let Intersection::InArea(v) | Intersection::OutsideArea(v) = intersection {
            let expected = match expected {
                Intersection::InArea(v) | Intersection::OutsideArea(v) => v,
                _ => panic!("Unexpected intersection type"),
            };
            assert!((v.x - expected.x).abs() < tolerance, "x: {}, expected: {}", v.x, expected.x);
            assert!((v.y - expected.y).abs() < tolerance, "y: {}, expected: {}", v.y, expected.y);
        }
    }

    #[test]
    fn test_intersection_in_the_past() {
        let hs = Hailstone::from_str("0, 0, 0 @ 2, 0, 0").unwrap();
        let pos_in_past = Position::new(-2.0, 0.0, 0.0);
        let pos_in_future = Position::new(2.0, 0.0, 0.0);

        assert!(hs.check_if_intersection_in_past(&pos_in_past));
        assert!(!hs.check_if_intersection_in_past(&pos_in_future));
    }
}