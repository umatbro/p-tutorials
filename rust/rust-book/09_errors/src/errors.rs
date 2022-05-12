use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

pub fn err() {
    let f = match File::open("hello.txt") {
        Ok(h) => h,
        Err(error) => panic!("Error occurred: {:?}", error),
    };
    println!("File {:?}", f)
}

pub fn handle_error() {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound  => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error)
            }
            other_error => panic!("Unknown error ocurred: {:?}", other_error)
        }
    };
    // print_file_content(&mut f);
}

fn _print_file_content(file: &mut File) -> &File {
    let mut data = String::new();
    let result = file.read_to_string(&mut data);
    match result {
        Ok(_) => println!("{}", data),
        Err(error) => panic!("Error while reading the file: {:?}", error),
    }
    return file;
}