use crate::pos::Direction;

fn parse_char_num(input: &str) -> Option<(char, i32)> {
    let mut chars = input.chars();
    let c = chars.next()?;
    let _ = chars.next()?; // skip space
    let num_str = chars.collect::<String>();
    let num = num_str.parse::<i32>().ok()?;
    Some((c, num))
}

pub fn parse_line(input: &str) -> Option<Vec<Direction>> {
    use Direction::*;

    let (c, num) = parse_char_num(input)?;
    let direction = match c {
        'D' => Down,
        'U' => Up,
        'L' => Left,
        'R' => Right,
        _ => return None,
    };

    Some(vec![direction; num as usize])
}