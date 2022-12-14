use serde_json::json;
use serde_json::Value;

use crate::parse::parse_line;

#[derive(Debug)]
pub struct Pair {
    left: Value,
    right: Value,
}

#[derive(Debug)]
enum PairComparisonError {
    NoDecision,
    WrongInput(String),
}
use PairComparisonError::*;

impl Pair {
    pub fn from(left: &str, right: &str) -> Self {
        Self {
            left: parse_line(left).unwrap(),
            right: parse_line(right).unwrap(),
        }
    }

    pub fn in_order(&self) -> bool {
        compare_values(&self.left, &self.right).unwrap()
    }
}

fn compare_values(left: &Value, right: &Value) -> Result<bool, PairComparisonError> {
    use Value::*;
    if let (Array(_), Array(_)) = (left, right) {
    } else {
        return Err(WrongInput("left and right are not both arrays".to_string()));
    }

    let mut left_arr = left.as_array().unwrap().clone();
    let mut right_arr = right.as_array().unwrap().clone();
    if left_arr.len() == 0 && right_arr.len() == 0 {
        return Err(NoDecision);
    }
    left_arr.reverse();
    right_arr.reverse();

    while let Some(left_item) = left_arr.pop() {
        let right_item = right_arr.pop();
        if let None = right_item {
            // If the right list runs out of items first, the inputs are not in the right order.
            return Ok(false);
        }
        let right_item = right_item.unwrap();

        // at this point, both left and right exist
        match (&left_item, &right_item) {
            (Number(lft), Number(rgh)) => {
                let l = lft.as_i64().unwrap();
                let r = rgh.as_i64().unwrap();
                if l == r {
                    if left_arr.is_empty() && right_arr.is_empty() {
                        return Err(NoDecision);
                    }
                    continue;
                } else {
                    return Ok(l < r);
                }
            }
            (Array(left), Array(right)) => {
                let res = compare_values(&left_item, &right_item);
                match &res {
                    Ok(val) => return Ok(*val),
                    Err(e) => match e {
                        NoDecision => continue,
                        _ => {
                            return res;
                        }
                    },
                }
            }
            (Number(left), Array(right)) => {
                let new_left = Array(Vec::from(vec![json!(left)]));
                return compare_values(&new_left, &right_item);
            }
            (Array(left), Number(right)) => {
                let new_right = json!([right]);
                return compare_values(&left_item, &new_right);
            }

            _ => {
                return Err(WrongInput(format!(
                    "Undexpected branch:\nL: {left_item}\nR: {right_item}"
                )));
            }
        }
    }

    // If the left list runs out of items first, the inputs are in the right order.
    Ok(true)
}

#[cfg(test)]
mod tests {
    // [1,1,3,1,1]
    // [1,1,5,1,1]

    // [[1],[2,3,4]]
    // [[1],4]

    // [9]
    // [[8,7,6]]

    // [[4,4],4,4]
    // [[4,4],4,4,4]

    // [7,7,7,7]
    // [7,7,7]

    // []
    // [3]

    // [[[]]]
    // [[]]

    // [1,[2,[3,[4,[5,6,7]]]],8,9]
    // [1,[2,[3,[4,[5,6,0]]]],8,9]

    use rstest::*;

    use super::Pair;

    #[rstest]
    #[case("[[],1]", "[[],2]", true)]
    #[case("[[],2]", "[[],1]", false)]
    #[case("[1,1,3,1,1]", "[1,1,5,1,1]", true)]
    #[case("[[1],[2,3,4]]", "[[1],4]", true)]
    #[case("[9]", "[[8,7,6]]", false)]
    #[case("[[4,4],4,4]", "[[4,4],4,4,4]", true)]
    #[case("[7,7,7,7]", "[7,7,7]", false)]
    #[case("[]", "[3]", true)]
    #[case("[[[]]]", "[[]]", false)]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", false)]
    #[case(
        "[[[[],[10,0,1],4]],[[0],7],[0,[],[]],[],[]]",
        "[[7,[[],[1],[5,5,6],[10,3],5],10,10],[[]],[5,[2,2],1,4]]",
        true
    )]
    #[case(
        "[[10,[10,[9,8,3],3,0],[[3,10,10],[5,8,5]],[],7],[],[8],[7,1]]",
        "[[7,1],[[2],[[3,1,7,8,3],7]]]",
        false
    )]
    #[case(
        "[[[5,[9,4,7,6],5,[8],[7,3]],[],[4],[[3,8],[8,8,9,6,9],[1,5,1]]],[[],5]]",
        "[[10,[8,[9,6],4,[6,0,3]]],[]]",
        true
    )]
    #[case("[[5],[6,2,[9,[1,3,9]]],[[8,[2,0,8,2],[],[8,6,2,9]],9,5],[3,0,3,8]]", "[[],[[],[],[],[6,8,[5,10,6],7],0],[[[0,5,7],[],[0,3,4,10,2],[1,0,1],[10]],8,[1,4]],[[[10,2,10,10,2],3,6,8,0],2,5]]", false)]
    #[case("[[5,3,6,9],[[7,9],8,[[1,8],0,[1],8,[]],10,6],[1],[2,[[7,2,5],[2,7]],[[],10,[],[5,10,1],10],[6,[1,1],[7,6,10,2,1],0],[]],[[8,[0],0],5,4,[],1]]", "[[[5,[0,8,0,9,0],[1,8,9],[1,9]],[8,4,[],[3,10,4],3],1,9,[]],[[1,10,2],6],[5,5,10],[[1,[0,5,0],[2,6,1],4,7],[[1]],[10,[1,0],2,[],4],[[8,8,7,8],[2,6]]]]", true)]
    #[case(
        "[[2,8]]",
        "[[[[5],[9,9,6],[1,8],[5,4,6,0,2]],[[5,9],[]],[7,[4,2,3,4],6]],[],[1,9,7,[6]]]",
        true
    )]
    #[case("[[5,[7]],[[6,3],[8,[]],6],[[[3,8,1,1,3]],1,7,[],0],[1,[],[],0],[]]", "[]", false)]
    fn test_compare_values(#[case] left: &str, #[case] right: &str, #[case] expected_result: bool) {
        let pair = Pair::from(left, right);
        let result = pair.in_order();
        assert_eq!(result, expected_result);
    }
}
