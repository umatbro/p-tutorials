use std::collections::btree_set::Intersection;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use iterchunks::IterChunks;

fn main() {
    let path = Path::new("input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    
    // Part 1
    // let mut result = 0;
    // for line in lines {
    //     let text = line.unwrap();
    //     println!("{}", text);
    //     let (left, right) = split_str(&text);
    //     let common = get_common(left, right);
    //     result += points_value(&common);
    // }
    // println!("Result is {}", result);
    let mut result = 0;
    for [a,b,c] in lines.into_iter().map(|el| el.unwrap()).array_chunks() {
       let common_in_3 = get_common_in_3(&a, &b, &c);
       let value = points_value(&common_in_3);
       result += value;
    }
    println!("Part 2 result is {}", result);
}

fn get_common(left: &str, right: &str) -> char {
    let left_s: HashSet<char> = left.chars().into_iter().collect();
    let right_s: HashSet<char> = right.chars().into_iter().collect();

    left_s.intersection(&right_s).next().unwrap().to_owned()
}

fn get_common_in_3(a: &String, b: &String, c: &String) -> char {
    let a_set: HashSet<char> = a.chars().into_iter().collect();
    let b_set: HashSet<char> = b.chars().into_iter().collect();
    let c_set: HashSet<char> = c.chars().into_iter().collect();

    let intersection = a_set.intersection(&b_set);
    let intersection: HashSet<char> = intersection.into_iter().map(|el| el.to_owned()).collect();
    let mut intersection = intersection.intersection(&c_set);
    
    intersection.next().unwrap().to_owned()
}

fn split_str(s: &String) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn points_value(ch: &char) -> u32 {
    let mut ctr = 1;
    let chars: HashMap<char, u32> = ('a'..='z').chain('A'..='Z').map(|c| {
        let v = ctr.clone();
        ctr += 1;
        (c, v)
    }).collect();
    chars.get(&ch).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::{get_common, points_value, split_str};

    #[test]
    fn test_get_common() {
        let result = 'p';
        let left = "vJrwpWtwJgWr";
        let right = "hcsFMMfFFhFp";
        assert_eq!(get_common(left, right), result);
    }

    #[test]
    fn test_split_str() {
        let s = "abcd".to_string();
        let result = split_str(&s);
        assert_eq!(result, ("ab", "cd"));
    }

    #[test]
    fn test_point_value() {
        let a = 'a';
        let A = 'A';
        let b = 'b';

        assert_eq!(points_value(&a), 1);
        assert_eq!(points_value(&b), 2);
        assert_eq!(points_value(&A), 27);
        assert_eq!(points_value(&'c'), 3);
        assert_eq!(points_value(&'Z'), 52);
    }
}
