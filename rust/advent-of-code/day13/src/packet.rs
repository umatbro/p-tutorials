use serde_json::Value;
use serde_json::json;

use crate::parse::parse_line;

pub struct Pair {
    left: Value,
    right: Value,
}

impl Pair {
    pub fn from(left: &str, right: &str) -> Self {
        Self {
            left: parse_line(left).unwrap(),
            right: parse_line(right).unwrap(),
        }
    }

    pub fn in_order(&self) -> bool {
        use Value::*;

        if let (Array(left), Array(right)) = (&self.left, &self.right) {
            let first = left.first();
            if first.is_none() {
                return true;
            }
            let right = right.first();
            if right.is_none() {
                return false;
            }
            return compare_values(first.unwrap(), first.unwrap())
        } else { panic!("Unexpected type") }

        false
    }
}

fn compare_values(left: &Value, right: &Value) -> bool {
    false
}