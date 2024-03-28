use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    thread::spawn(move || {
        s1.send("Hello World!").unwrap();
        for message in r2 {
            println!("Thread1 Received: {}", message);
            s1.send("Hello World!").unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for message in r1 {
            println!("Thread2 Received: {}", message);
            s2.send("Good Bye!").unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    thread::sleep(Duration::from_secs(10))
}