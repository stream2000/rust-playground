#[cfg(test)]
mod test {
    use std::fs::metadata;
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use std::thread::JoinHandle;
    use crate::concurrent;
    use std::sync::mpsc;

    #[test]
    fn cps() {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];
        let handle1 = thread::spawn(move || {
            tx.send(String::from("hello")).unwrap()
        });
        handles.push(handle1);

        let handle2 = thread::spawn(move || {
            let data = rx.recv().unwrap();
            println!("{}", data)
        });
        handles.push(handle2);

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn shared_memory() {
        // New a mutex counter
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        let counter1 = Arc::clone(&counter);
        let handle1 = thread::spawn(move || {
            let mut data = counter1.lock().unwrap();
            *data += 1;
        });
        handles.push(handle1);

        let counter2 = Arc::clone(&counter);
        let handle2 = thread::spawn(move || {
            let mut data = counter2.lock().unwrap();
            *data += 1;
            println!("data:{}", *data)
        });
        handles.push(handle2);

        handles.reverse();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}