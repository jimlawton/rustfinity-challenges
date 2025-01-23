use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn create_shared_data<T>(initial: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(initial))
}

pub fn increment_counter(
    counter: Arc<Mutex<i32>>,
    threads: usize,
    increments: usize,
) -> Vec<JoinHandle<()>> {
    let mut v = Vec::<JoinHandle<()>>::new();
    for _ in 0..threads {
        let ctr = counter.clone();
        v.push(thread::spawn(move || {
            let mut c = ctr.lock().unwrap();
            *c += increments as i32;
        }));
    }
    v
}

pub fn modify_shared_data<T: Send + 'static>(
    data: Arc<Mutex<T>>,
    modifier: fn(&mut T),
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut d = data.lock().unwrap();
        modifier(&mut (*d));
    })
}

// Example usage
pub fn main() {
    let counter = create_shared_data(0);
    let handles = increment_counter(Arc::clone(&counter), 5, 10);
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter value: {}", *counter.lock().unwrap());

    let shared_string = create_shared_data(String::from("Hello"));
    let handle = modify_shared_data(shared_string.clone(), |s| s.push_str(" World"));
    handle.join().unwrap();
    println!("Modified string: {}", *shared_string.lock().unwrap());
}

