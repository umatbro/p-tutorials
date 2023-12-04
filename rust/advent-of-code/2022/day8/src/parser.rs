use nom::{
    bytes::complete::{take_while_m_n, take_while}, character::{complete::digit1, is_digit}, combinator::map_res, IResult,
};

fn single_digit(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, d) = take_while_m_n(1, 1, is_digit)(input)?;
    let num = d[0] as char;
    Ok((input, num.to_string().parse::<u32>().unwrap()))
}

#[cfg(test)]
mod tests {
    use nom::InputIter;

    use super::single_digit;

    #[test]
    fn test_get_single_digit() {
        let input = b"2137420";

        let (i, digit) = single_digit(input).unwrap();
        assert_eq!(digit, 2);
        let (i, digit) = single_digit(i).unwrap();
        assert_eq!(digit, 1);
        let (i, digit) = single_digit(i).unwrap();
        assert_eq!(digit, 3);
        let (i, digit) = single_digit(i).unwrap();
        assert_eq!(digit, 7);
        let (i, digit) = single_digit(i).unwrap();
        assert_eq!(digit, 4);
        let (i, digit) = single_digit(i).unwrap();
        assert_eq!(digit, 2);
        let (i, digit) = single_digit(i).unwrap();
        assert_eq!(digit, 0);
    }
}
