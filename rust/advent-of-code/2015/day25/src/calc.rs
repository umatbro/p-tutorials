pub fn calc_code(prev_code: &u64) -> u64 {
    prev_code * 252533 % 33554393
}


#[cfg(test)]
mod tests {
    use crate::calc::calc_code;

    #[test]
    fn test_calc_code() {
        let first = 20151125;
        let r1 = calc_code(&first);
        assert_eq!(r1, 31916031);
        let r2 = calc_code(&r1);
        assert_eq!(r2, 18749137);
        let r3 = calc_code(&r2);
        assert_eq!(r3, 16080970);
        let r4 = calc_code(&r3);
        assert_eq!(r4, 21629792);
        let r5 = calc_code(&r4);
        assert_eq!(r5, 17289845);
    }
}