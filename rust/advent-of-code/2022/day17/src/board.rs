use std::{collections::HashSet, fmt::Display, cmp::max};

use crate::{shape::{Shape, ShapeType}, coord::Coord};
use self::ShapeType::*;

const HEIGHT: i32 = 7;

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

pub struct Board {
    formation: HashSet<Coord>,
    falling_rock: Option<Shape>,
    curr_shape_type: ShapeType,
    pub rocks_stopped: u64  ,
}

impl Board {
    pub fn new() -> Self {
        Self {
            formation: HashSet::new(),
            falling_rock: None,
            curr_shape_type: ShapeType::Square,
            rocks_stopped: 0,
        }
    }

    pub fn get_max_height(&self) -> i32 {
        match self.formation.iter().map(|r| r.y).max() {
            Some(v) => v+1,
            None => 0,
        }
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
        self.falling_rock = Some(Shape::new(spawn_pos, next_shape_type.clone()));
        self.curr_shape_type = next_shape_type;
    }

    fn clean(&mut self) {
        let max = self.get_max_height();
        let lower_bound = max - HEIGHT;
        self.formation.retain(|c| c.y > lower_bound);
    }

    pub fn play_round(&mut self, direction: Direction) {
        if let None = self.falling_rock {
            self.spawn_falling_rock()
        }
        let move_result = match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        };

        match move_result {
            Ok(_) => (),
            Err(e) => assert!(e.eq(&MoveError::HitWall) || e.eq(&MoveError::HitRock)),
        }

        let down_res = self.move_down();
        match down_res {
            Ok(_) => (),
            Err(e) => assert!(e.eq(&MoveError::HitFloor) || e.eq(&MoveError::HitRock)),
        }

        self.clean();
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
            if x > 5 {
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

    fn move_down(&mut self) -> Result<(), MoveError> {
        let shape = self.falling_rock.as_ref().unwrap();
        let coords = shape.get_coords();

        let height_when_rock_should_be_deleted = self.get_max_height() - HEIGHT - 4;
        
        for c in coords.iter() {
            if c.y == 0 || c.y <= height_when_rock_should_be_deleted {
                self.formation.extend(coords);
                self.falling_rock = None;
                self.rocks_stopped += 1;
                return Err(MoveError::HitFloor);
            }
        }
        let new_coords: HashSet<Coord> = coords.iter().map(|c| Coord::new(c.x, c.y -1)).collect();
        for c in new_coords.iter() {
            if self.formation.contains(c) {
                self.formation.extend(coords);
                self.falling_rock = None;
                self.rocks_stopped += 1;
                return Err(MoveError::HitRock);
            }
        }
        let curr_pos = self.falling_rock.as_ref().unwrap().get_pos();
        let new_pos = curr_pos.relative(0, -1);
        self.falling_rock.as_mut().unwrap().set_pos(new_pos);
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let falling_rock_coords = match &self.falling_rock {
            Some(r) => r.get_coords(),
            None => HashSet::new(),
        };
        let formation_coords: HashSet<Coord> = self.formation.iter().map(|c| c.relative(0, 0)).collect();
        let combined_coords: HashSet<_> = falling_rock_coords.union(&formation_coords).collect();

        let max_height = combined_coords.iter().map(|c| c.y).max().unwrap_or(1);
        let lower_bound = max(0, max_height - HEIGHT - 4);
        for line in (lower_bound..=max_height).rev() {
            for col in 0..7 {
                let this_coord = Coord::new(col, line);
                let coord_in_falling = falling_rock_coords.contains(&this_coord);
                let coord_in_formation = formation_coords.contains(&this_coord);
                if coord_in_falling {
                    write!(f, "{}", "@").unwrap();
                } else if coord_in_formation {
                    write!(f, "{}", "#").unwrap();
                } else {
                    write!(f, "{}", ".").unwrap();
                }
            }
            writeln!(f, "  // {}", line).unwrap();
        }
        writeln!(f, "Fallen: {}", self.rocks_stopped).unwrap();
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