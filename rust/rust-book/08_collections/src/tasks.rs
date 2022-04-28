use std::collections::HashMap;

const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
//['a', 'e', 'i', 'o', 'u', 'y'];

// Given a list of integers, use a vector and return:
// * the median  (when sorted, the value in the middle position)
// * the mode mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn task1(vec: &Vec<i32>) {
    println!("The list is {:?}", vec);

    println!("Median is: {}", median(vec));
    println!("Mode is: {}", mode(vec).expect("Mode not found"));
}

fn median(vec: &Vec<i32>) -> i32 {
    return vec[vec.len() / 2];
}

fn mode(data: &Vec<i32>) -> Option<i32> {
    let mut scores: HashMap<i32, i32> = HashMap::new();
    for val in data.iter() {
        let count = scores.entry(*val).or_insert(0);
        *count += 1;
    }

    let max_by = scores.iter().max_by(|a, b| a.1.cmp(&b.1));
    println!("Max by: {:?}", max_by);
    let val = max_by.map(|(k, _v)| k);
    println!("Sorted: {:?}", val);
    val.copied()
}

pub fn task2(s: &String) -> String {
    let first_char = s.chars().next().unwrap();
    let mut result = String::new();
    if !is_vowel(&first_char) {
        result = format!("{}-{}{}", &s[1..], first_char, "ay");
        result
    } else {
        result.push_str(s);
        result.push_str("-hay");
        result
    }
}

fn is_vowel(ch: &char) -> bool {
    VOWELS.contains(&ch.to_lowercase().next().unwrap())
}
