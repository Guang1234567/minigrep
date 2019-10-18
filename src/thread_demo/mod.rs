

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);

        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }


    #[test]
    fn it_works2() {
        assert_eq!(2, 2);

        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn it_works_channel() {
        assert_eq!(2, 2);
        let (tx, rx) = mpsc::channel();

        // 生产者线程
        let producer = thread::spawn(move || {
            let val = String::from("hi");
            println!("将要发送的数据 : {:?}", &val);
            match tx.send(val) {
                Ok(()) => println!("发送成功!"),
                Err(e) => println!("发送失败! {:?}", e),
            }
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn it_works_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..1024{
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
}