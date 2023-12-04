use std::collections::HashSet;

pub fn parse_range(s: &str) -> HashSet<u32> {
    let split = s
        .split("-")
        .take(2)
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let [low, high] = <[u32; 2]>::try_from(split).ok().unwrap();

    HashSet::from_iter(low..=high)
}

pub fn parse_row(row: &str) -> (&str, &str) {
    let split: [&str; 2] = row
        .split(",")
        .take(2)
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let [left, right] = split;
    (left, right)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::range::parse_row;

    use super::parse_range;

    #[test]
    fn test_parse_range() {
        let input1 = "2-6";
        assert_eq!(parse_range(input1), HashSet::from([2, 3, 4, 5, 6]));

        let input2 = "7-9";
        assert_eq!(parse_range(input2), HashSet::from([7, 8, 9]));
    }

    #[test]
    fn test_row() {
        let input1 = "2-4,6-8";
        assert_eq!(parse_row(input1), ("2-4", "6-8"));
        let input1 = "5-7,7-9";
        assert_eq!(parse_row(input1), ("5-7", "7-9"));
    }
}
