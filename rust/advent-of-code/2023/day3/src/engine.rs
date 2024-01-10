use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct SymbolPosition {
    row: usize,
    column: usize,
}

impl SymbolPosition {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn find_adjacent_parts<'a>(&self, parts: &'a Vec<PartNumber>) -> Vec<&'a PartNumber> {
        let mut adjacent_parts = Vec::new();
        for part in parts {
            let part_position = &part.position;
            if part_position
                .has_symbol_in_adjacency(&vec![SymbolPosition::new(self.row, self.column)])
            {
                adjacent_parts.push(part);
            }
        }

        adjacent_parts
    }
}

#[derive(Debug, PartialEq)]
pub struct PartPosition {
    row: usize,
    column: usize,
    length: usize,
}

impl PartPosition {
    fn new(row: usize, column: usize, length: usize) -> Self {
        Self {
            row,
            column,
            length,
        }
    }

    fn has_symbol_in_adjacency(&self, symbols: &Vec<SymbolPosition>) -> bool {
        symbols
            .iter()
            .filter(|symbol| {
                let s_row = symbol.row;
                let s_column = symbol.column;

                let column_min = match self.column {
                    0 => 0,
                    _ => self.column - 1,
                };
                let column_max = self.column + self.length + 1;

                (self.row).abs_diff(s_row) <= 1 && (column_min..column_max).contains(&s_column)
            })
            .count()
            > 0
    }
}

#[derive(Debug, PartialEq)]
pub struct PartNumber {
    pub number: u32,
    position: PartPosition,
}

impl PartNumber {
    pub fn has_symbol_in_adjacency(&self, symbols: &Vec<SymbolPosition>) -> bool {
        self.position.has_symbol_in_adjacency(symbols)
    }
}

#[derive(Debug, PartialEq)]
pub struct Engine {
    pub symbols: Vec<SymbolPosition>,
    pub parts: Vec<PartNumber>,
}

impl FromStr for Engine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = Vec::new();
        let mut parts = Vec::new();
        for (row, line) in s.lines().enumerate() {
            // the number to track how many iterations to skip, after we've found a first digit of a number
            let mut skip_next_iterations = 0;
            for (col, char) in line.chars().enumerate() {
                if skip_next_iterations > 0 {
                    skip_next_iterations -= 1;
                    continue;
                }
                match char {
                    '.' | '\n' => (),
                    '0'..='9' => {
                        let consecutive_digits = line
                            .chars()
                            .skip(col)
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>();
                        let length = consecutive_digits.len();
                        let number = consecutive_digits.parse::<u32>().unwrap();
                        let part = PartNumber {
                            number,
                            position: PartPosition::new(row, col, length),
                        };
                        parts.push(part);
                        skip_next_iterations = length - 1;
                    }
                    _ => {
                        symbols.push(SymbolPosition::new(row, col));
                    }
                }
            }
        }
        Ok(Engine { symbols, parts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;

    #[rstest]
    #[case(PartPosition::new(0, 0, 1), vec![SymbolPosition::new(0, 1)], true)]
    #[case(PartPosition::new(0, 0, 1), vec![SymbolPosition::new(0, 2)], false)]
    #[case(PartPosition::new(2, 2, 2), vec![SymbolPosition::new(7, 3), SymbolPosition::new(1, 7)], false)]
    #[case(PartPosition::new(2, 2, 2), vec![SymbolPosition::new(7, 3), SymbolPosition::new(1, 3)], true)]
    fn test_has_symbol_in_adjacency(
        #[case] part: PartPosition,
        #[case] symbols: Vec<SymbolPosition>,
        #[case] expected: bool,
    ) {
        assert_eq!(part.has_symbol_in_adjacency(&symbols), expected);
    }

    #[test]
    fn test_parse_engine() {
        let mut string = String::from("...345...\n");
        string.push_str("..*......\n");
        string.push_str("...67....\n");
        let engine = string.parse::<Engine>().unwrap();
        assert_eq!(engine.symbols.len(), 1);
        assert_eq!(engine.parts.len(), 2, "Parts: {:?}", engine.parts);

        assert_eq!(
            engine,
            Engine {
                symbols: vec![SymbolPosition::new(1, 2)],
                parts: vec![
                    PartNumber {
                        number: 345,
                        position: PartPosition::new(0, 3, 3)
                    },
                    PartNumber {
                        number: 67,
                        position: PartPosition::new(2, 3, 2)
                    },
                ]
            }
        );
    }

    #[rstest]
    #[case(
        SymbolPosition::new(1, 3),
        vec![
            PartNumber {number: 467, position: PartPosition::new(0, 0, 3)},
            PartNumber {number: 35, position: PartPosition::new(2, 2, 2)},
        ],
    )]
    fn find_parts_adjacent_to_symbol(
        #[case] symbol: SymbolPosition,
        #[case] expected: Vec<PartNumber>,
    ) {
        let engine = fs::read_to_string("input.test")
            .expect("Something went wrong reading the file")
            .parse::<Engine>()
            .unwrap();
        let adjacent_parts = symbol.find_adjacent_parts(&engine.parts);
        assert_eq!(adjacent_parts.len(), expected.len());
        for i in 0..adjacent_parts.len() {
            assert_eq!(adjacent_parts[i], &expected[i]);
        }
    }
}
