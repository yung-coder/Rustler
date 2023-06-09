use std::{thread, time::Duration};

fn main() {
    let handel = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned no rustling", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} for main", i);
        thread::sleep(Duration::from_millis(1));
    }

    // blocking  main thread tiil complete
    handel.join().unwrap();
}
