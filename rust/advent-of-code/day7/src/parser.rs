use std::str::FromStr;

use nom::{IResult, bytes::complete::tag, combinator, character::complete::{digit1, space1}};

#[derive(PartialEq, Debug)]
pub enum OperationType {
    Cd(String),
    Ls,
}

#[derive(PartialEq, Debug)]
pub enum ElementType {
    Dir {
        name: String,
    },
    File {
        name: String,
        size: u64,
    }
}

#[derive(PartialEq, Debug)]
pub enum ParseResult {
    Operation(OperationType),
    Element(ElementType),
}

pub fn parse(input: &str) -> IResult<&str, ParseResult> {
    use ParseResult::*;
    use OperationType::*;
    use ElementType::*;
    use nom::error;

    let result = tag::<_, _, error::Error<&str>>("$ ")(input);
    if let Ok((command, _match)) = result {
        if let Ok((i, _ls))= tag::<_, _, error::Error<&str>>("ls")(command) {
            return Ok((i, Operation(Ls)))
        }
        let (dir, _cd) = tag("cd ")(command)?;
        return Ok((dir, Operation(Cd(dir.to_string()))));
    }

    if let Ok((dirname, _dir_prefix)) = tag::<_, _, error::Error<&str>>("dir ")(input) {
        return Ok((
            dirname,
            Element(Dir {
                name: dirname.to_string(),
            })
        ));
    }
    let (i, size) = combinator::map_res(digit1, u64::from_str)(input)?;
    let (filename, _) = space1(i)?;
    Ok((
        filename,
        Element(File{
            name: filename.to_string(),
            size,
        })
    ))
}

#[cfg(test)]
mod test {
    use crate::parser::parse;
    use crate::parser::ParseResult::*;
    use crate::parser::OperationType::*;
    use crate::parser::ElementType::*;

    #[test]
    fn test_parse_cd() {
        let input = "$ cd /";
        let (_, result) = parse(input).unwrap();
        assert_eq!(result, Operation(Cd(String::from("/"))));
    }

    #[test]
    fn test_parse_ls() {
        let input = "$ ls";
        let (_, result) = parse(input).unwrap();
        assert_eq!(result, Operation(Ls));
    }

    #[test]
    fn test_parse_ls_dir() {
        let input = "dir a";
        let (_, result) = parse(input).unwrap();
        assert_eq!(Element(Dir {
            name: String::from("a"),
        }), result);
    }

    #[test]
    fn test_parse_file() {
        let input = "14848514 b.txt";
        let (_, result) = parse(input).unwrap();
        assert_eq!(result, Element(File {
            name: String::from("b.txt"),
            size: 14848514,
        }));
    }
}