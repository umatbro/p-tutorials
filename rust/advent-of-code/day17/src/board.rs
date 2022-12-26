use std::{collections::HashSet};

use crate::{shape::{Shape, ShapeType}, coord::Coord};
use self::ShapeType::*;


pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
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

    fn move_left(&mut self) -> Result<(), MoveError> {
        let shape = self.falling_rock.as_ref().unwrap();
        let current_coords = shape.get_coords();

        for c in current_coords {
            let x = c.x;
            if x < 1 {
                return Err(MoveError::HitWall);
            }
            let new_coord = Coord::new(c.x - 1, c.y);
            if self.formation.contains(&new_coord) {
                return Err(MoveError::HitRock);
            }
        }

        let s = self.falling_rock.as_mut().unwrap();
        s.pos = s.pos.relative(-1, 0);
        
        Ok(())
    }

    fn move_right(&mut self) -> Result<(), MoveError> {
        let shape = self.falling_rock.as_ref().unwrap();
        let current_coords = shape.get_coords();
        
        for c in current_coords {
            let x = c.x;
            if x > 6 {
                return Err(MoveError::HitWall);
            }
            let new_coord = Coord::new(c.x + 1, c.y);
            if self.formation.contains(&new_coord) {
                return Err(MoveError::HitRock);
            }
        }


        let s = self.falling_rock.as_mut().unwrap();
        s.pos = s.pos.relative(1, 0);
        
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::board::MoveError;
    use crate::coord::Coord;

    use super::Board;
    use super::Direction::*;

    #[test]
    fn test_move_left() {
        let mut board = Board::new();
        board.spawn_falling_rock();
        board.move_left().unwrap();

        assert_eq!(board.falling_rock.unwrap().get_pos(), &Coord::new(1, 3));
    }

    #[test]
    fn test_move_right() {
        let mut board = Board::new();
        board.spawn_falling_rock();
        board.move_right().unwrap();

        assert_eq!(board.falling_rock.unwrap().get_pos(), &Coord::new(3, 3));
    }

    #[test]
    fn test_rock_collision_left() {
        let mut board = Board::new();
        board.spawn_falling_rock();
        board.formation = HashSet::from_iter([Coord::new(1, 3)]);
        let r = board.move_left();
        assert!(r.is_err());
        let e = r.err().unwrap();
        assert_eq!(e, MoveError::HitRock);
    }

    #[test]
    fn test_rock_collision_right() {
        let mut board = Board::new();
        board.spawn_falling_rock();
        board.formation = HashSet::from_iter([Coord::new(6, 3)]);
        
        let r = board.move_right();
        assert!(r.is_err());
        let e = r.err().unwrap();
        assert_eq!(e, MoveError::HitRock);
    }
}