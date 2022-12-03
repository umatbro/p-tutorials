#[derive(Debug)]
pub struct PaperPos {
    row: u64,
    col: u64,
}

impl Clone for PaperPos {
    fn clone(&self) -> Self {
        Self { row: self.row.clone(), col: self.col.clone() }
    }
}
impl PartialEq for PaperPos {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}
impl Eq for PaperPos{}
impl PaperPos {
    pub fn from(row: u64, col: u64) -> Self {
        Self {row, col}
    }
}

pub struct InfinitePaper {
    diag_len: u64,
    current_pos: PaperPos,
}
impl InfinitePaper {
    pub fn new() -> Self {
        Self {diag_len: 1, current_pos: PaperPos::from(1, 1)}
    }
}
impl Iterator for InfinitePaper {
    type Item = PaperPos;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.current_pos.clone();
        if self.current_pos.col == self.diag_len {
            assert!(self.current_pos.row == 1);
            self.diag_len += 1;
            self.current_pos = PaperPos::from(self.diag_len, 1);
        } else {
            self.current_pos.col += 1;
            self.current_pos.row -= 1;
        }

        Some(return_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::traverse::PaperPos;

    use super::InfinitePaper;

    #[test]
    fn test_iterate() {
        let mut paper = InfinitePaper::new();
        assert_eq!(paper.next().unwrap(), PaperPos::from(1, 1));
        assert_eq!(paper.next().unwrap(), PaperPos::from(2, 1));
        assert_eq!(paper.next().unwrap(), PaperPos::from(1, 2));
        assert_eq!(paper.next().unwrap(), PaperPos::from(3, 1));
        assert_eq!(paper.next().unwrap(), PaperPos::from(2, 2));
        assert_eq!(paper.next().unwrap(), PaperPos::from(1, 3));
        assert_eq!(paper.next().unwrap(), PaperPos::from(4, 1));
        assert_eq!(paper.next().unwrap(), PaperPos::from(3, 2));
        assert_eq!(paper.next().unwrap(), PaperPos::from(2, 3));
        assert_eq!(paper.next().unwrap(), PaperPos::from(1, 4));
    }
}