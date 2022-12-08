use std::fmt::Display;

pub struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            trees: vec![]
        }
    }

    pub fn add_row(&mut self, row: Vec<u32>) -> &Self {
        self.trees.push(row);
        self
    }

    pub fn get_pos_value(&self, row_num: usize, pos_in_row: usize) -> u32 {
        self.trees[row_num][pos_in_row]
    }

    pub fn is_visible(&self, row_num: usize, pos_in_row: usize) -> bool {
        if row_num == 0 || row_num == self.trees.len() - 1 {
            return true;
        }
        if pos_in_row == 0 || pos_in_row == self.trees[row_num].len() - 1 {
            return true;
        }

        let my_val = self.get_pos_value(row_num, pos_in_row);
        let views = vec![
            self.look_from_top(row_num, pos_in_row),
            self.look_from_left(row_num, pos_in_row),
            self.look_from_right(row_num, pos_in_row),
            self.look_from_bottom(row_num, pos_in_row),
        ];

        views.iter().any(|view| is_tree_visible(&my_val, view))
    }

    pub fn viewscore(&self, row_num: usize, pos_in_row: usize) -> u32 {
        let my_val = self.get_pos_value(row_num, pos_in_row);
        let top_score = view_score_on_side(&my_val, &self.look_top(row_num, pos_in_row));
        let left_score = view_score_on_side(&my_val, &self.look_left(row_num, pos_in_row));
        let right_score = view_score_on_side(&my_val, &self.look_right(row_num, pos_in_row));
        let bottom_score = view_score_on_side(&my_val, &self.look_botton(row_num, pos_in_row));

        top_score * left_score * right_score * bottom_score
    }

    fn look_from_top(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        let cap = if row_num < 1 { 0 } else { row_num - 1 };
        let mut result = Vec::with_capacity(cap);

        for row_n in 0..row_num {
            result.push(self.get_pos_value(row_n, pos_in_row));
        }
        result
    }

    fn look_from_left(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        let cap = if pos_in_row < 1 { 0 } else { pos_in_row - 1 };
        let mut result = Vec::with_capacity(cap);

        for pos_in_r in 0..pos_in_row {
            result.push(self.get_pos_value(row_num, pos_in_r));
        }
        result
    }

    fn look_from_bottom(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        let cap = self.trees.len() - row_num - 1;
        let mut result = Vec::with_capacity(cap);
        for row_n in (row_num+1..self.trees.len()).rev() {
            result.push(self.get_pos_value(row_n, pos_in_row));
        }
        result
    }

    fn look_from_right(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        let row_len = self.trees[row_num].len();
        let cap = row_len - pos_in_row - 1;
        let mut result = Vec::with_capacity(cap);
        for col_n in (pos_in_row+1..row_len).rev() {
            result.push(self.get_pos_value(row_num, col_n));
        }
        result
    }

    pub fn count_visible(&self) -> u32 {
        let mut result = 0;
        for (row_num, row) in self.trees.iter().enumerate() {
            for (col_num, _) in row.iter().enumerate() {
                if self.is_visible(row_num, col_num) {
                    result += 1;
                }
            }
        };
        result
    }

    fn look_right(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        self.look_from_right(row_num, pos_in_row).into_iter().rev().collect()
    }
    fn look_left(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        self.look_from_left(row_num, pos_in_row).into_iter().rev().collect()
    }
    fn look_top(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        self.look_from_top(row_num, pos_in_row).into_iter().rev().collect()
    }
    fn look_botton(&self, row_num: usize, pos_in_row: usize) -> Vec<u32> {
        self.look_from_bottom(row_num, pos_in_row).into_iter().rev().collect()
    }
    pub fn find_max_viewscore(&self) -> u32 {
        let mut max = 0;
        for (i, row) in self.trees.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let vs = self.viewscore(i, j);
                if vs > max {
                    max = vs;
                }
            }
        }
        
        max
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.trees.iter() {
            for item in row.iter() {
                s.push_str(&item.to_string())
            }
            s.push_str("\n");
        }
        let s = s.trim();
        write!(f, "{}", s)
    }
}

fn is_tree_visible(my_val: &u32, trees_in_front: &Vec<u32>) -> bool {
    let max_in_front = trees_in_front.iter().max();
    match max_in_front {
        Some(v) => my_val > v,
        None => true,
    }
}

fn view_score_on_side(my_val: &u32, trees_in_view: &Vec<u32>) -> u32 {
    let mut result = 0;
    for tree in trees_in_view.iter() {
        result += 1;
        if tree >= my_val {
            break;
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use crate::forest::Forest;
    use rstest::*;

    use super::view_score_on_side;

    #[fixture]
    fn example_forest() -> Forest {
        let mut forest = Forest::new();
        forest.add_row(vec![3, 0, 3, 7, 3]);
        forest.add_row(vec![2, 5, 5, 1, 2]);
        forest.add_row(vec![6, 5, 3, 3, 2]);
        forest.add_row(vec![3, 3, 5, 4, 9]);
        forest.add_row(vec![3, 5, 3, 9, 0]);

        forest
    }

    #[rstest]
    fn test_display(example_forest: Forest) {
        let s = "30373
25512
65332
33549
35390";
        assert_eq!(s, example_forest.to_string());
    }

    #[rstest]
    // trees on border
    #[case(0, 0, 3)]
    #[case(0, 1, 0)]
    #[case(0, 2, 3)]
    #[case(0, 3, 7)]
    #[case(0, 4, 3)]
    #[case(1, 0, 2)]
    #[case(2, 0, 6)]
    #[case(3, 0, 3)]
    #[case(4, 0, 3)]
    #[case(4, 1, 5)]
    #[case(4, 2, 3)]
    #[case(4, 3, 9)]
    #[case(4, 4, 0)]
    #[case(3, 4, 9)]
    #[case(2, 4, 2)]
    #[case(1, 4, 2)]
    // trees in the middle
    #[case(1, 1, 5)]
    #[case(1, 2, 5)]
    #[case(1, 3, 1)]
    #[case(2, 1, 5)]
    #[case(2, 2, 3)]
    #[case(2, 3, 3)]
    #[case(3, 1, 3)]
    #[case(3, 2, 5)]
    #[case(3, 3, 4)]
    fn test_get_pos_value(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_val: u32) {
        let result = example_forest.get_pos_value(row_num, pos_in_row);
        assert_eq!(result, expected_val);
    }

    #[rstest]
    // trees on border
    #[case(0, 0, true)]
    #[case(0, 1, true)]
    #[case(0, 2, true)]
    #[case(0, 3, true)]
    #[case(0, 4, true)]
    #[case(1, 0, true)]
    #[case(2, 0, true)]
    #[case(3, 0, true)]
    #[case(4, 0, true)]
    #[case(4, 1, true)]
    #[case(4, 2, true)]
    #[case(4, 3, true)]
    #[case(4, 4, true)]
    #[case(3, 4, true)]
    #[case(2, 4, true)]
    #[case(1, 4, true)]
    // trees in the middle
    #[case(1, 1, true)]
    #[case(1, 2, true)]
    #[case(1, 3, false)]
    #[case(2, 1, true)]
    #[case(2, 2, false)]
    #[case(2, 3, true)]
    #[case(3, 1, false)]
    #[case(3, 2, true)]
    #[case(3, 3, false)]
    fn test_is_visible(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] should_be_visible: bool) {
        let result = example_forest.is_visible(row_num, pos_in_row);
        assert_eq!(result, should_be_visible);
    }

    #[rstest]
    #[case(0, 0, vec![])]
    #[case(1, 0, vec![3])]
    #[case(4, 3, vec![7, 1, 3, 4])]
    #[case(3, 1, vec![0, 5, 5])]
    fn test_look_from_top(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_view: Vec<u32>) {
        let res = example_forest.look_from_top(row_num, pos_in_row);
        assert_eq!(res, expected_view);
    }

    #[rstest]
    #[case(0, 0, vec![])]
    #[case(1, 0, vec![])]
    #[case(4, 3, vec![3, 5, 3])]
    #[case(3, 1, vec![3])]
    fn test_look_from_left(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_view: Vec<u32>) {
        let res = example_forest.look_from_left(row_num, pos_in_row);
        assert_eq!(res, expected_view);
    }

    #[rstest]
    #[case(0, 0, vec![3, 3, 6, 2])]
    #[case(1, 0, vec![3, 3, 6])]
    #[case(4, 3, vec![])]
    #[case(3, 1, vec![5])]
    fn test_look_from_bottom(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_view: Vec<u32>) {
        let res = example_forest.look_from_bottom(row_num, pos_in_row);
        assert_eq!(res, expected_view);
    }

    #[rstest]
    #[case(0, 0, vec![3, 7, 3, 0])]
    #[case(1, 0, vec![2, 1, 5, 5])]
    #[case(4, 3, vec![0])]
    #[case(3, 1, vec![9, 4, 5])]
    fn test_look_from_right(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_view: Vec<u32>) {
        let res = example_forest.look_from_right(row_num, pos_in_row);
        assert_eq!(res, expected_view);
    }

    #[rstest]
    fn test_count_visible(example_forest: Forest) {
        assert_eq!(example_forest.count_visible(), 21);
    }

    #[rstest]
    #[case(0, 0, vec![0, 3, 7, 3])]
    #[case(1, 0, vec![5, 5, 1, 2])]
    #[case(4, 3, vec![0])]
    #[case(3, 1, vec![5, 4, 9])]
    fn test_look_right(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_view: Vec<u32>) {
        let res = example_forest.look_right(row_num, pos_in_row);
        assert_eq!(res, expected_view);
    }

    #[rstest]
    #[case(5, vec![4, 9], 2)]
    #[case(5, vec![3], 1)]
    #[case(5, vec![4,3, 2, 1], 4)]
    #[case(5, vec![4, 4, 5, 6, 7], 3)]
    #[case(5, vec![], 0)]
    fn test_view_score_on_side(#[case] my_val: u32, #[case] trees_in_view: Vec<u32>, #[case] expected_result: u32) {
        let result = view_score_on_side(&my_val, &trees_in_view);
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(3, 2, 8)]
    #[case(1, 2, 4)]
    fn test_viewscore(example_forest: Forest, #[case] row_num: usize, #[case] pos_in_row: usize, #[case] expected_viewscore: u32) {
        let result = example_forest.viewscore(row_num, pos_in_row);
        assert_eq!(result, expected_viewscore);
    }
}
