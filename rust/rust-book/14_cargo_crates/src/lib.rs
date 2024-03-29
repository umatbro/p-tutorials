//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// Adds one to the number given.
/// 
/// # Examples
/// ```
/// use cargo_crates::add_one;
/// 
/// let arg = 5;
/// let answer = add_one(arg);
/// assert_eq!(6, answer)
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
