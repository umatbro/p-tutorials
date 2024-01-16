use std::str::FromStr;
use std::fmt::{Debug, Formatter};

#[derive(PartialEq)]
pub struct MapRule {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

impl MapRule {
    pub fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    pub fn get_destination_value(&self, source_value: u64) -> u64 {
        match (self.source_range_start..self.source_range_start + self.range_length)
            .contains(&source_value)
        {
            true => self.destination_range_start + (source_value - self.source_range_start),
            false => source_value,
        }
    }
}

impl FromStr for MapRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split_whitespace();
        let destination_range_start = values
            .next()
            .ok_or("Missing source range start")?
            .parse::<u64>()
            .map_err(|e| format!("Invalid source range start: {}", e))?;
        let source_range_start = values
            .next()
            .ok_or("Missing destination range start")?
            .parse::<u64>()
            .map_err(|e| format!("Invalid destination range start: {}", e))?;
        let range_length = values
            .next()
            .ok_or("Missing range length")?
            .parse::<u64>()
            .map_err(|e| format!("Invalid range length: {}", e))?;
        Ok(Self::new(
            destination_range_start,
            source_range_start,
            range_length,
        ))
    }
}

impl Debug for MapRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.destination_range_start, self.source_range_start, self.range_length)
    }
}

#[derive(Debug)]
pub struct Mapper {
    map_rules: Vec<MapRule>,
}

impl Mapper {
    pub fn new(map_rules: Vec<MapRule>) -> Self {
        Self { map_rules }
    }

    pub fn map(&self, source_value: u64) -> u64 {
        for map_rule in &self.map_rules {
            let result = map_rule.get_destination_value(source_value);
            if result != source_value {
                return result;
            }
        }
        source_value
    }
}

impl FromStr for Mapper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_rules = s
            .lines()
            .skip(1)
            .map(|line| line.parse::<MapRule>())
            .collect::<Result<Vec<MapRule>, String>>()?;
        Ok(Self::new(map_rules))
    }
}

pub struct MapperChain {
    pub list_of_seeds: Vec<u64>,
    pub mappers: Vec<Mapper>,
}

impl FromStr for MapperChain {
    type Err = String;

    /// Example:
    ///
    /// seeds: 79 14 55 13
    ///
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    ///
    /// soil-to-fertilizer map:
    /// 0 15 37
    /// 37 52 2
    /// 39 0 15
    ///
    /// fertilizer-to-water map:
    /// 49 53 8
    /// 0 11 42
    /// 42 0 7
    /// 57 7 4
    ///
    /// water-to-light map:
    /// 88 18 7
    /// 18 25 70
    ///
    /// light-to-temperature map:
    /// 45 77 23
    /// 81 45 19
    /// 68 64 13
    ///
    /// temperature-to-humidity map:
    /// 0 69 1
    /// 1 0 69
    ///
    /// humidity-to-location map:
    /// 60 56 37
    /// 56 93 4
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let list_of_seeds = lines
            .next()
            .ok_or("Missing list of seeds")?
            .replace("seeds:", "")
            .split_whitespace()
            .map(|seed| seed.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .unwrap();
        lines.next().unwrap(); // skip empty line
        let mappers = lines
            .collect::<Vec<&str>>()
            .join("\n")
            .split("\n\n")
            .map(|section| section.parse::<Mapper>())
            .collect::<Result<Vec<Mapper>, _>>()
            .unwrap();
        Ok(Self {
            list_of_seeds,
            mappers,
        })
    }
}

impl MapperChain {
    pub fn map_seed(&self, seed: u64) -> u64 {
        let mut result = seed;
        for mapper in &self.mappers {
            result = mapper.map(result)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("50 98 2", MapRule::new(50, 98, 2))]
    #[case("52 50 48", MapRule::new(52, 50, 48))]
    #[case("0 15 37", MapRule::new(0, 15, 37))]
    #[case("37 52 2", MapRule::new(37, 52, 2))]
    #[case("39 0 15", MapRule::new(39, 0, 15))]
    #[case("49 53 8", MapRule::new(49, 53, 8))]
    #[case("0 11 42", MapRule::new(0, 11, 42))]
    #[case("42 0 7", MapRule::new(42, 0, 7))]
    #[case("57 7 4", MapRule::new(57, 7, 4))]
    fn test_map_rule_from_str(#[case] input: &str, #[case] expected: MapRule) {
        let result = input.parse::<MapRule>().unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(MapRule::new(50, 98, 2), 98, 50)]
    #[case(MapRule::new(50, 98, 2), 99, 51)]
    #[case(MapRule::new(50, 98, 2), 100, 100)]
    #[case(MapRule::new(52, 50, 48), 53, 55)]
    #[case(MapRule::new(52, 50, 48), 96, 98)]
    #[case(MapRule::new(52, 50, 48), 10, 10)]
    fn test_map_rule_get_destination_value(
        #[case] map_rule: MapRule,
        #[case] source_value: u64,
        #[case] expected: u64,
    ) {
        let result = map_rule.get_destination_value(source_value);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(vec![MapRule::from_str("50 98 2").unwrap(), MapRule::from_str("52 50 48").unwrap()], 79, 81)]
    #[case(vec![MapRule::from_str("50 98 2").unwrap(), MapRule::from_str("52 50 48").unwrap()], 14, 14)]
    #[case(vec![MapRule::from_str("50 98 2").unwrap(), MapRule::from_str("52 50 48").unwrap()], 55, 57)]
    #[case(vec![MapRule::from_str("50 98 2").unwrap(), MapRule::from_str("52 50 48").unwrap()], 13, 13)]
    #[case(vec!["0 15 37", "37 52 2", "39 0 15"].iter().map(|s| MapRule::from_str(s).unwrap()).collect(), 81, 81)]
    #[case(vec!["0 15 37", "37 52 2", "39 0 15"].iter().map(|s| MapRule::from_str(s).unwrap()).collect(), 14, 53)]
    #[case(vec!["0 15 37", "37 52 2", "39 0 15"].iter().map(|s| MapRule::from_str(s).unwrap()).collect(), 57, 57)]
    #[case(vec!["0 15 37", "37 52 2", "39 0 15"].iter().map(|s| MapRule::from_str(s).unwrap()).collect(), 13, 52)]
    fn test_mapper_map(
        #[case] map_rules: Vec<MapRule>,
        #[case] source_value: u64,
        #[case] expected: u64,
    ) {
        let mapper = Mapper::new(map_rules);
        let result = mapper.map(source_value);
        assert_eq!(result, expected);
    }
}
