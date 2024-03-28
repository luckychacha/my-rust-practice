use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;

        if let Ok(mut started) = lock.try_lock() {
            *started = true;
            eprintln!("I'm a happy thread!");
            cvar.notify_all();

            loop {
                thread::sleep(Duration::from_secs(1));
                println!("Working...");
            }
        } else {
            eprintln!("I'm a sad thread!");
        }
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    eprintln!("Worker Started!");

}