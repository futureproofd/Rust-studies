use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;
use std::vec;

fn main() {
    // spawn a second thread (closure)
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //handle.join().unwrap(); <- or call it here to have it finish first, before the main thread

    for i in 1..5 {
        println!("number {} from main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
    // Blocking a thread means that thread is prevented from performing work or exiting.
    handle.join().unwrap();

    // Using move Closures with threads
    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        // <-- without the 'move' keyword, we're only borrowing
        // this results in an error because the borrow reference may fall out of scope.
        // 'move' forces the closure to take ownership of the values it's using
        println!("vector is {:?}", v);
    });
    handle2.join().unwrap();

    /*
        Transmitter / Receiver (tx, rx) using mpsc (multiple producer, single consumer)
    */
    // using analogies, think of a channel as a river channel
    // we have multiple transmitters (tx) and a single receiver (tx)
    let (tx, rx) = mpsc::channel();
    // create a second producer
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hey"),
            String::from("from"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            // println!("val is {}",val); <-- this won't work because we've already sent it down the channel (safe concurrency)
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("from"),
            String::from("second thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            // println!("val is {}",val); <-- this won't work because we've already sent it down the channel (safe concurrency)
            thread::sleep(Duration::from_secs(1));
        }
    });

    // instead of calling recv, treat receiver as an iterator
    for received in rx {
        println!("Got {}", received);
    }
}
