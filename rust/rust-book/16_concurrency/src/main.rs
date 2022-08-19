use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Number from thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join();
    for i in (1..=5) {
        println!("Number from main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let l = vec![1, 3, 4];
    let handle = thread::spawn(move || {
        println!("List from thread: {:?}", l);
    });
    handle.join().unwrap();

    messaging();
}

fn messaging() {
    // mpsc - Multiple Producer Single Consumer
    let txrx = mpsc::channel();
    let tx: Sender<i32> = txrx.0;
    let rx: Receiver<i32> = txrx.1;
}

