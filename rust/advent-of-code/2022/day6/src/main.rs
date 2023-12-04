use std::fs::File;
use std::collections::HashSet;
use std::io::{self, Seek};
use std::io::{BufReader, Read};

fn main() -> io::Result<()>{
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);
    let result = find_marker(& mut reader, 4);

    println!("Result for task 1 is {result}");

    reader.rewind().unwrap();
    let result2 = find_marker(&mut reader, 14);
    println!("Result for task 2 is {result2}");
    Ok(())
}

fn find_marker<R: Read>(reader: &mut BufReader<R>, distinct_chars: usize) -> u32 {
    let mut reader_buffer = [0; 1];
    let mut buffer: Vec<char> = Vec::new();
    let mut current_pos = 0;
    // fill buf with first 4 characters
    for i in 0..distinct_chars {
        reader.read(&mut reader_buffer).expect(&format!("Failed to initialize buffer on pos {i}"));
        buffer.push(reader_buffer[0] as char);
        current_pos += 1;
    }

    while let Ok(_) = reader.read(&mut reader_buffer) {
        let set: HashSet<&char> = HashSet::from_iter(buffer.iter());
        // println!("Set len is {} ({:?})", set.len(), set);
        if set.len() == distinct_chars {
            break;
        }
        current_pos += 1;
        buffer.drain(..1);
        assert_eq!(buffer.len(), distinct_chars - 1);
        buffer.push(reader_buffer[0] as char);
        assert_eq!(buffer.len(), distinct_chars);
        // println!("Current buffer is {:?}", buffer);
    }

    current_pos
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::find_marker;

    #[test]
    fn case1() {
        let mut reader = BufReader::new("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes());
        let result = find_marker(&mut reader, 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn case2() {
        let mut reader = BufReader::new("nppdvjthqldpwncqszvftbrmjlhg".as_bytes());
        let result = find_marker(&mut reader, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn case3() {
        let mut reader = BufReader::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes());
        let result = find_marker(&mut reader, 4);
        assert_eq!(result, 10);
    }

    #[test]
    fn case4() {
        let mut reader = BufReader::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes());
        let result = find_marker(&mut reader, 4);
        assert_eq!(result, 11);
    }

    #[test]
    fn case5() {
        let mut reader = BufReader::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes());
        let result = find_marker(&mut reader, 14);
        assert_eq!(result, 19);
    }

    #[test]
    fn case6() {
        let mut reader = BufReader::new("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes());
        let result = find_marker(&mut reader, 14);
        assert_eq!(result, 23);
    }
    #[test]
    fn case7() {
        let mut reader = BufReader::new("nppdvjthqldpwncqszvftbrmjlhg".as_bytes());
        let result = find_marker(&mut reader, 14);
        assert_eq!(result, 23);
    }
    #[test]
    fn case8() {
        let mut reader = BufReader::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes());
        let result = find_marker(&mut reader, 14);
        assert_eq!(result, 29);
    } 
}
