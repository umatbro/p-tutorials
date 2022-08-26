use std::cell::RefCell;
use std::sync::{Mutex, Arc};
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
    channels();
    shared_state();
}

fn messaging() {
    // mpsc - Multiple Producer Single Consumer
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        println!("Starting the transmitter");
        let val = String::from("hi");
        thread::sleep(Duration::from_secs(1));
        tx.send(val).unwrap();
    });

    let received = rx.recv().expect("Could not get the value");
    println!("got: {}", received);
}


fn channels() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(format!("(t2) {}", val)).unwrap();
            thread::sleep(Duration::from_secs_f32(0.8));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let arc = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = arc.lock().unwrap();
            *num += 1;
            println!("Thread {} is setting value to {}", i, num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}