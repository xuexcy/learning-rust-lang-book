// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn multi_thread_with_mutex() {
    // Rc 不能保证在多线程下改变计数的原子性
    // let counter = Rc::new(Mutex::new(0));
    // Atomic Reference Counted
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move|| {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
fn main() {
    let m = Mutex::new(5);
    {   // 如果其他线程拥有锁并panic，那么没人能再获取锁
        // 所以这里要用unwrap来处理遇到panic的情况
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    multi_thread_with_mutex();
}
