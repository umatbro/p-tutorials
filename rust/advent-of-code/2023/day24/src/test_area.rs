use crate::hailstone::Position;

pub struct TestArea {
    pub bound_low: i64,
    pub bound_high: i64,
}

impl TestArea {
    pub fn new(bound_low: i64, bound_high: i64) -> Self {
        Self { bound_low, bound_high }
    }

    pub fn is_in_area(&self, position: &Position) -> bool {
        position.x >= self.bound_low as f64 && position.x <= self.bound_high as f64 && position.y >= self.bound_low as f64 && position.y <= self.bound_high as f64
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::TestArea;
    use crate::hailstone::Position;

    #[rstest]
    #[case(Position::new(14.333, 15.333, 0.0), TestArea::new(7, 27), true)]
    #[case(Position::new(6.2, 19.4, 0.0), TestArea::new(7, 27), false)]
    #[case(Position::new(-6.0, -5.0, 0.0), TestArea::new(7, 27), false)]
    #[case(Position::new(16.0, -5.0, 0.0), TestArea::new(7, 27), false)]
    fn test_is_in_area(#[case] position: Position, #[case] area: TestArea, #[case] expected: bool) {
        assert_eq!(area.is_in_area(&position), expected);
    }
}