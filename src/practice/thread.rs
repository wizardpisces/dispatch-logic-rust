use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// fn channel() -> JoinHandle<()> {
fn channel() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let join_handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    for received in rx {
        println!("Got: {}", received);
    }

    join_handle.join().unwrap();


    // join_handle
}

/**
 * 使用 Rc<T> 就有造成引用循环的风险，这时两个 Rc<T> 值相互引用，造成内存泄漏。同理，Mutex<T> 也有造成 死锁（deadlock） 的风险。
 * 这发生于当一个操作需要锁住两个资源而两个线程各持一个锁，这会造成它们永远相互等待。
 */
fn mutex() {
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

    println!("Result: {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_channel() {
        // channel().join();
        channel()
    }
    #[test]
    fn test_mutex() {
        mutex()
    }
}
