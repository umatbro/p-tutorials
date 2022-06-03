#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

#[derive(Debug)]
struct Rectangle<T> {
    x: T,
    y: T,
}

impl <T: PartialOrd> Rectangle<T> {
    fn can_hold(&self, other: &Rectangle<T>) -> bool {
        self.x > other.x && self.y > other.y
    }
}

mod tests_rectangle {
    use super::{Rectangle};
    
    #[test]
    fn test_ints() {
        let r = Rectangle{x: 2, y: 2};
        let other = Rectangle{x: 1, y: 1};
        assert!(r.can_hold(&other))
    }
    
    #[test]
    fn test_strings() {
        let r = Rectangle{x: String::from("2"), y: String::from("2")};
        let other = Rectangle{x: String::from("1"), y: String::from("1")};
        assert!(r.can_hold(&other));
        assert_eq!(other.can_hold(&r), false);
    }
}
