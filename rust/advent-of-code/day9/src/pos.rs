use std::ops::Add;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        self.chebyshev_distance(other) <= 1
    }

    pub fn chebyshev_distance(&self, other: &Self) -> u32 {
        let dx = (self.x - other.x).abs() as u32;
        let dy = (self.y - other.y).abs() as u32;

        cmp::max(dx, dy)
    }
}

impl Add<&Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Direction) -> Self::Output {
        use Direction::*;

        match rhs {
            Up => Pos::new(self.x, self.y + 1),
            Down => Pos::new(self.x, self.y - 1),
            Right => Pos::new(self.x + 1, self.y),
            Left => Pos::new(self.x - 1, self.y),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Right, // +x
    Left,  // -x
    Up,    // +y
    Down,  // -y
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::Direction;
    use super::Direction::*;
    use super::Pos;

    #[rstest]
    #[case((0, 0), (0, 0), true)]
    #[case((1, 0), (0, 0), true)]
    #[case((1, 0), (0, 1), true)]
    #[case((2, 0), (0, 0), false)]
    #[case((3, 0), (0, 0), false)]
    #[case((1, 1), (0, 0), true)]
    #[case((6,8), (8, 7), false)]
    fn test_is_adjacent(
        #[case] pos1: (i32, i32),
        #[case] pos2: (i32, i32),
        #[case] is_adjacent: bool,
    ) {
        let pos1 = Pos::new(pos1.0, pos1.1);
        let pos2 = Pos::new(pos2.0, pos2.1);

        assert_eq!(pos1.is_adjacent(&pos2), is_adjacent);
    }

    #[rstest]
    #[case(Up, (0, 1))]
    #[case(Down, (0, -1))]
    #[case(Right, (1, 0))]
    #[case(Left, (-1, 0))]
    fn test_add_direction(#[case] direction: Direction, #[case] final_pos: (i32, i32)) {
        let pos = Pos::new(0, 0);
        let pos = pos + &direction;
        assert_eq!((pos.x, pos.y), final_pos);
    }
}
