use std::str::FromStr;

type Millisecond = u64;
type Millimeter = u64;

#[derive(Debug, PartialEq)]
pub struct RaceRecord {
    time: Millisecond,
    distance: Millimeter,
}

impl RaceRecord {
    pub fn new(time: Millisecond, distance: Millimeter) -> Self {
        Self { time, distance }
    }

    pub fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .map_while(|time_to_hold| self.calculate_distance(time_to_hold).ok())
            .filter(|distance| distance > &self.distance)
            .count() as u64
    }

    pub fn calculate_distance(&self, time_to_hold: Millisecond) -> Result<Millimeter, String> {
        if time_to_hold > self.time {
            return Err(format!(
                "Cannot hold for {}, because time for race is {}",
                time_to_hold, self.time
            ));
        }
        let speed = time_to_hold;
        let time_to_travel = self.time - time_to_hold;
        Ok(speed * time_to_travel)
    }
}

#[derive(Debug, PartialEq)]
pub struct Races(pub Vec<RaceRecord>);

impl FromStr for Races {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number_pairs: Vec<Vec<u64>> = Vec::with_capacity(2);
        for line in s.lines() {
            // clear the ine from the "Time: " and "Distance: " prefixes
            let line = line
                .split(": ")
                .nth(1)
                .ok_or_else(|| format!("Invalid line: {}", line));
            let line = match line {
                Ok(line) => line,
                Err(err) => return Err(err),
            };
            // split the line by whitespace and parse the numbers
            let numbers = line
                .split_whitespace()
                .filter_map(|part| part.parse::<u64>().ok())
                .collect();
            number_pairs.push(numbers);
        }

        match number_pairs.len() {
            2 => {
                let race_records = number_pairs[0]
                    .iter()
                    .zip(&number_pairs[1])
                    .map(|(time, distance)| RaceRecord::new(*time, *distance))
                    .collect();
                Ok(Races(race_records))
            }
            _ => Err(format!(
                "Invalid number of lines: {}, expected 2",
                number_pairs.len()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_races() {
        let lines = ["Time:      7  15   30", "Distance:  9  40  200"];
        let input = lines.join("\n");
        let result = input.parse::<Races>().unwrap();
        assert_eq!(
            result,
            Races(vec![
                RaceRecord::new(7, 9),
                RaceRecord::new(15, 40),
                RaceRecord::new(30, 200),
            ])
        );
    }

    #[rstest]
    #[case(RaceRecord::new(7, 9), 4)]
    #[case(RaceRecord::new(15, 40), 8)]
    #[case(RaceRecord::new(30, 200), 9)]
    fn test_ways_to_win(#[case] race_record: RaceRecord, #[case] ways_to_win_expected: u64) {
        let result = race_record.ways_to_win();
        assert_eq!(result, ways_to_win_expected);
    }

    #[rstest]
    #[case(RaceRecord::new(7, 9), 0, Some(0))]
    #[case(RaceRecord::new(7, 9), 1, Some(6))]
    #[case(RaceRecord::new(7, 9), 2, Some(10))]
    #[case(RaceRecord::new(7, 9), 3, Some(12))]
    #[case(RaceRecord::new(7, 9), 4, Some(12))]
    #[case(RaceRecord::new(7, 9), 5, Some(10))]
    #[case(RaceRecord::new(7, 9), 6, Some(6))]
    #[case(RaceRecord::new(7, 9), 7, Some(0))]
    #[case(RaceRecord::new(7, 9), 8, None)]
    #[case(RaceRecord::new(7, 9), 9, None)]
    fn test_calculate_distance(
        #[case] race_record: RaceRecord,
        #[case] time_to_hold: Millisecond,
        #[case] expected_result: Option<Millimeter>,
    ) {
        let result = race_record.calculate_distance(time_to_hold).ok();
        assert_eq!(result, expected_result);
    }
}
