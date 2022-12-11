use crate::monke::{DivisionElement, MonkeInspection, OpType};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space0;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::separated_list0;
use nom::sequence;
use nom::sequence::tuple;
use nom::Err as NomErr;
use nom::IResult;

fn parse_division_element(input: &str) -> IResult<&str, DivisionElement<u32>> {
    let (input, el) = alt((tag("old"), digit1))(input)?;
    let el = match el {
        "old" => DivisionElement::Old,
        val => DivisionElement::Val(val.parse::<u32>().unwrap()),
    };
    Ok((input, el))
}

pub fn parse_inspection(input: &str) -> IResult<&str, MonkeInspection<u32>> {
    let input = input.trim();
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, first_el) = parse_division_element(input)?;
    let (input, op_type) = alt((tag(" * "), tag(" + ")))(input)?;
    let op_type = match op_type {
        " + " => OpType::Add,
        " * " => OpType::Mul,
        _ => return Err(NomErr::Error(nom::error::Error::new(input, ErrorKind::Tag))),
    };
    let (input, (_, second_element)) = sequence::tuple((space0, parse_division_element))(input)?;

    Ok((
        input,
        MonkeInspection {
            element1: first_el,
            element2: second_element,
            op_type,
        },
    ))
}

fn parse_comma_delimited_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(tag(", "), map_res(digit1, str::parse))(input)
}

pub fn parse_starting_items(input: &str) -> IResult<&str, Vec<u32>> {
    let input = input.trim();
    let (input, _) = tag("Starting items: ")(input)?;
    parse_comma_delimited_numbers(input)
}

pub fn parse_divisible_by(input: &str) -> IResult<&str, u32> {
    let input = input.trim();
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, number) = digit1(input)?;

    Ok((input, number.parse::<u32>().unwrap()))
}

pub fn if_true_target_parser(input: &str) -> IResult<&str, usize> {
    let input = input.trim();
    let (input, (_, number)) = tuple((tag("If true: throw to monkey "), digit1))(input)?;

    Ok((input, number.parse::<usize>().unwrap()))
}

pub fn if_false_target_parser(input: &str) -> IResult<&str, usize> {
    let input = input.trim();

    let (input, (_, number)) = tuple((tag("If false: throw to monkey "), digit1))(input)?;
    Ok((input, number.parse::<usize>().unwrap()))
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::parse_starting_items;

    #[rstest]
    #[case("  Starting items: 79, 98", vec![79, 98])]
    #[case("  Starting items: 54, 65, 75, 74", vec![54, 65, 75, 74])]
    #[case("  Starting items: 74", vec![74])]
    fn test_parse_starting_items(#[case] input: &str, #[case] expected_result: Vec<u32>) {
        let (_, result) = parse_starting_items(input).unwrap();
        assert_eq!(expected_result, result);
    }
}
