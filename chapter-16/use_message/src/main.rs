use std::{sync::mpsc, thread};
use std::time::Duration;

fn send_more() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move|| {
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
    for rec in rx {
        println!("Got: {}", rec);
    }
}

fn send_after_clone() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move|| {
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
    thread::spawn(move|| {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });
    for rec in rx {
        println!("Got: {}", rec);
    }
}

fn main() {
    // mpsc : multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    thread::spawn(move|| {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    send_more();
    send_after_clone();
}
