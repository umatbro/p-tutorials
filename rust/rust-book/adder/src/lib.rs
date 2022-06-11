#[cfg(test)]
mod tests {
    use super::Rectangle;
    use super::add_two;
    use super::internal_adder;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    #[test]
    #[should_panic(expected="Make this test fail")]
    fn should_panic() {
        // We place the #[should_panic] attribute after the #[test] attribute
        // and before the test function it applies to.
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{x: 8, y: 7};
        let smaller = Rectangle{x: 5, y: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
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
