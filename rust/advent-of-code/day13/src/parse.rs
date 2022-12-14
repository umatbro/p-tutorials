use serde_json::Value;

pub fn parse_line(input: &str) -> Result<Value, serde_json::Error> {
    let result = serde_json::from_str(input);
    result
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    use super::parse_line;
    use serde_json::json;
    use serde_json::Value;

    use Value::*;

    #[test]
    fn test_parse_line() {
        let input = "[[1],4]";
        let result = parse_line(input).unwrap();
        assert_eq!(result, Array(vec![Array(vec![json!(1)]), json!(4)]),);
    }
}
