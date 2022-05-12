use std::fs::File;
use std::io::{self, Read};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

pub fn read_username_q_operator() -> Result<(String, String), io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s1 = String::new();
    f.read_to_string(&mut s1)?;
    
    let mut s2 = String::new();
    File::open("hello.txt")?.read_to_string(&mut s2)?;
    Ok((s1, s2))
}