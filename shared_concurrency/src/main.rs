use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }
    println!("the value of m is: {}", *m.lock().unwrap());

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("The value of counter is: {}", *counter.lock().unwrap())
}
