mod task;
#[macro_use]
extern crate pest_derive;
extern crate alloc;

use pest::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct NumberParser;

/// The line consists of alphanumeric characters.
/// The `number` in result is two digit number consisting of
/// the first number character in the line and the last number character in the line.
fn find_number_in_line(line: &str) -> i32 {
    let mut first_number: Option<i32> = None;
    let mut last_number: i32 = 0;
    for c in line.chars() {
        if c.is_numeric() {
            if first_number == None {
                first_number = Some(c.to_digit(10).unwrap() as i32);
            }
            last_number = c.to_digit(10).unwrap() as i32;
        }
    }
    first_number.unwrap() * 10 + last_number
}

fn find_all_digits_in_line(line: &str) -> Vec<i32> {
    let mut digits_in_line = Vec::new();
    let mut buffer = String::new();
    for c in line.chars() {
        buffer.push(c);
        if c.is_numeric() {
            digits_in_line.push(c.to_digit(10).unwrap() as i32);
            continue
        }
        if buffer.ends_with("one") {
            digits_in_line.push(1);
        } else if buffer.ends_with("two") {
            digits_in_line.push(2);
        } else if buffer.ends_with("three") {
            digits_in_line.push(3);
        } else if buffer.ends_with("four") {
            digits_in_line.push(4);
        } else if buffer.ends_with("five") {
            digits_in_line.push(5);
        } else if buffer.ends_with("six") {
            digits_in_line.push(6);
        } else if buffer.ends_with("seven") {
            digits_in_line.push(7);
        } else if buffer.ends_with("eight") {
            digits_in_line.push(8);
        } else if buffer.ends_with("nine") {
            digits_in_line.push(9);
        } else if buffer.ends_with("zero") {
            digits_in_line.push(0);
        }
    }
    digits_in_line
}

fn find_line_number_p2(digits: &Vec<i32>) -> i32 {
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn find_numbers_with_words(task: &String) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for (i, line) in task.lines().enumerate() {
        let digits_in_line = find_all_digits_in_line(line);
        let number_to_add = find_line_number_p2(&digits_in_line);
        numbers.push(number_to_add);
    }
    numbers
}

fn find_numbers(task: &String) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for line in task.lines() {
        let number = find_number_in_line(line);
        numbers.push(number);
    }
    numbers
}

fn main() {
    println!("part1");
    let task = task::read_task("input");
    let numbers = find_numbers(&task);
    let sum = numbers.iter().sum::<i32>();
    println!("Sum: {}", sum);

    println!("part2");
    let numbers = find_numbers_with_words(&task);
    let sum = numbers.iter().sum::<i32>();
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::task;
    use super::find_all_digits_in_line;
    use super::find_line_number_p2;
    use rstest::rstest;

    #[test]
    fn test_part_2() {
        let data = task::read_task("part2-test.txt");
        let numbers = super::find_numbers_with_words(&data);
        let sum = numbers.iter().sum::<i32>();
        assert_eq!(sum, 281);
    }

    #[rstest(line, expected_digits, expected_number)]
    #[case("59sixfivefive", vec![5, 9, 6, 5, 5], 55)]
    #[case("9five9six8threet", vec![9, 5, 9, 6, 8, 3], 93)]
    #[case("vksix4fourvrbfmjcrhb78", vec![6, 4, 4, 7, 8], 68)]
    #[case("22two9", vec![2, 2, 2, 9], 29)]
    #[case("nineonedmlffz5", vec![9, 1, 5], 95)]
    #[case("bdnhvtsjmdnklsxbtmnztqjtpnz6fivesevenfourzddgsrfmlq", vec![6, 5, 7, 4], 64)]
    #[case("562", vec![5, 6, 2], 52)]
    #[case("7xf", vec![7], 77)]
    #[case("fiveb1", vec![5, 1], 51)]
    #[case("8k", vec![8], 88)]
    #[case("treb7uchet", vec![7], 77)]
    #[case("5nss", vec![5], 55)]
    #[case("7pqrstsixteen", vec![7, 6], 76)]
    #[case("dmhkvgbc6four6eightwofkk", vec![6, 4, 6, 8, 2], 62)]
    #[case("vdxoneight6", vec![1, 8, 6], 16)]
    fn test_line(line: &str, expected_digits: Vec<i32>, expected_number: i32) {
        let result = find_all_digits_in_line(line);
        assert_eq!(result, expected_digits);
        let result = find_line_number_p2(&result);
        assert_eq!(result, expected_number);
    }
}
