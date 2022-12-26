use std::collections::HashSet;

use crate::{shape::{Shape, ShapeType}, coord::Coord};
use self::ShapeType::*;


pub enum Direction {
    Left,
    Right,
}

pub enum MoveError {
    HitFloor,
    HitWall,
    HitRock,
}

pub fn make_move(shape: &Shape, direction: Direction) -> Result<HashSet<Coord>, MoveError> {
    let current_coords = shape.get_coords();
    todo!()
}

pub struct Board {
    formation: HashSet<Coord>,
    falling_rock: Option<Shape>,
    curr_shape_type: ShapeType
}

impl Board {
    pub fn new() -> Self {
        Self {
            formation: HashSet::new(),
            falling_rock: None,
            curr_shape_type: ShapeType::Square,
        }
    }

    pub fn get_max_height(&self) -> i32 {
        self.formation.iter().map(|r| r.y).max().unwrap_or(0)
    }

    fn spawn_falling_rock(&mut self) {
        assert!(self.falling_rock.is_none());
        let max_height = self.get_max_height();
        let spawn_pos = Coord::new(2, max_height + 3);
        let next_shape_type = match self.curr_shape_type {
            ShapeType::HorizontalLine => Cross,
            ShapeType::Cross => ReverseL,
            ShapeType::ReverseL => VerticalLine,
            ShapeType::VerticalLine => Square,
            ShapeType::Square => HorizontalLine,
        };
        self.falling_rock = Some(Shape::new(spawn_pos, next_shape_type));
    }

    pub fn play_round(&mut self, direction: Direction) {
        if let None = self.falling_rock {
            self.spawn_falling_rock()
        }
        // match direction {
        //     Direction::Left => self.move_left(),
        //     Direction::Right => self.move_right(),
        // };
        

    }
}