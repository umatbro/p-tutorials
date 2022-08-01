use rand;

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn add_to_one() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
