use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashSet, hash::Hash,
};

use crate::coord::Coord;


pub enum ShapeType {
    HorizontalLine,
    Cross,
    ReverseL,
    VerticalLine,
    Square,
}

pub struct Shape {
    pub shape_type: ShapeType,
    pub pos: Coord,
}

impl Shape {
    pub fn new(pos: Coord, shape_type: ShapeType) -> Self {
        Self {pos, shape_type}
    }
    pub fn get_pos(&self) -> &Coord {
        &self.pos
    }
    pub fn set_pos(&mut self, new_pos: Coord) {
        self.pos = new_pos;
    }

    pub fn get_coords(&self) -> HashSet<Coord> {
        let coords = match &self.shape_type {
            ShapeType::HorizontalLine => HashSet::from_iter([
                self.pos.relative(0, 0),
                self.pos.relative(1, 0),
                self.pos.relative(2, 0),
                self.pos.relative(3, 0),
            ]),
            ShapeType::Cross => HashSet::from_iter([
                self.pos.relative(1, 0),
                self.pos.relative(0, 1),
                self.pos.relative(0, 1),
                self.pos.relative(1, 1),
                self.pos.relative(2, 1),
                self.pos.relative(1, 2),
            ]),
            ShapeType::ReverseL => HashSet::from_iter([
                self.pos.relative(0, 0),
                self.pos.relative(1, 0),
                self.pos.relative(2, 0),
                self.pos.relative(2, 1),
                self.pos.relative(2, 2),
            ]),
            ShapeType::VerticalLine => HashSet::from_iter([
                self.pos.relative(0, 0),
                self.pos.relative(0, 1),
                self.pos.relative(0, 2),
                self.pos.relative(0, 3),
            ]),
            ShapeType::Square => HashSet::from_iter([
                self.pos.relative(0, 0),
                self.pos.relative(1, 0),
                self.pos.relative(1, 1),
                self.pos.relative(0, 1),
            ]),
        };
        coords
    }
}


pub fn collides(shape: &Shape, other: &Shape) -> bool {
    let coord_shape = shape.get_coords();
    let coord_other = other.get_coords();

    !coord_shape.is_disjoint(&coord_other)
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::collides;
    use crate::{coord::Coord, shape::Shape};
    use super::ShapeType::*;

    #[test]
    fn test_cross_new() {
        let cross = Shape::new(Coord::new(0, 0), Cross);
        assert_eq!(
            HashSet::from_iter([
                Coord::new(1, 1),
                Coord::new(2, 1),
                Coord::new(1, 2),
                Coord::new(0, 1),
                Coord::new(1, 0),
            ]),
            cross.get_coords()
        );
    }

    #[test]
    fn test_collision() {
        let cross = Shape::new(Coord::new(0, 0), Cross);
        let line = Shape::new(Coord::new(0, 1), VerticalLine);
        assert!(collides(&cross, &line));
    }
}
