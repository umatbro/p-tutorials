use std::{
    process,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use server_project::ThreadPool;

const CRLF: &str = "\r\n";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = match ThreadPool::build(4) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.0);
            process::exit(1);
        },
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    //    let http_request: Vec<_> = buf_reader
    //        .lines()
    //        .map(|result| result.unwrap())
    //        .take_while(|line| !line.is_empty())
    //        .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let lines: Vec<String> = vec![
        String::from(status_line),
        String::from(format!("Content-Length: {length}{CRLF}")),
        contents,
        String::from(CRLF),
    ];

    stream.write_all(lines.join(CRLF).as_bytes()).unwrap();
}
