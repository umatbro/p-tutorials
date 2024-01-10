use std::fs;

pub fn read_task(file_name: &str) -> String {
    println!("Reading task from file: {}", file_name);
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}
