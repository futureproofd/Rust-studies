use std::sync::{mpsc, Arc, Mutex};
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

    /*
        Mutex - or Mutual exclusion
        allows only one thread to access some data at any given time.
        To access the data in a mutex, a thread must first signal that it
        wants access by asking to acquire the mutex’s lock. The lock is
        a data structure that is part of the mutex that keeps track of who currently
        has exclusive access to the data. Therefore, the mutex is described as guarding
        the data it holds via the locking system.
    */

    let m = Mutex::new(5);
    {
        // You must attempt to acquire the lock before using the data.
        let mut num = m.lock().unwrap();
        *num = 6; // <-- note the dereference operator which gets a reference to the value (see smart-pointers)
                  // When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
                  // here we get that by default as it goes out of scope, using Drop (which is built into the MutexGuard)

        println!("m = {:?}", m);
    }

    /*
     Shared state concurrency
    */
    // first attempt: Give counter multiple owners by using the smart pointer Rc<T> to create a reference counted value.
    // Unfortunately, Rc<T> is not safe to share across threads.
    // When Rc<T> manages the reference count, it adds to the count for each
    // call to clone and subtracts from the count when each clone is dropped. (but doesn't use any concurrency for performance reasons)
    // we use the thread-safe Arc (atomic reference counter) instead
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // clone the lock
        let counter = Arc::clone(&counter);
        // create 10 threads
        let handle = thread::spawn(move || {
            // aquire lock
            let mut num = counter.lock().unwrap(); // <---interior mutability
                                                   // increase deref value
            *num += 1;
            // close scope and release lock
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // We get a result of 10
    println!("Result: {}", *counter.lock().unwrap());
}
