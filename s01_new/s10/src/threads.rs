/**

**/

#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    #[test]
    fn test1() {
        // Arc 运行时，允许多个线程同时访问数据
        // Mutex 运行时，只允许一个线程访问数据
        // thread::spawn() 运行时，开启一个线程
        // .join() 运行时，等待线程结束

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


    // channel
    #[test]
    fn test2() {
        let (tx, rx) = mpsc::channel();
        let tx2 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        thread::spawn(move || {
            let val = String::from("h2");
            tx2.send(val).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
        // let received = rx.recv().unwrap();
        // println!("Got: {}", received);
    }
}