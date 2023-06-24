use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

/// A toy ThreadPool implementation
pub struct ThreadPool {
    workers: Vec<Worker>,
    // We need owner ship of sender when drooping the thread pool
    // However, vars in the struct must stay, so we use an Option
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // basic method to create a channel
        let (sender, receiver) = mpsc::channel();

        // Why we need arc + mutex? It's easy to understand~
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Here we use type alias, we can also try to use tuple struct in other situation?
type Job = Box<dyn FnOnce() + Send + 'static>;

// How to hold data inside a struct? Sometimes we use hashmap(which is a key-value pair
// and sometimes we can use tuple directly.
struct ThreadInfo(i32, String);

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // NOTE: the return value of `lock` is dropped after this line of expression
            // So we do the unlock behaviour just after this line
            let message = receiver.lock().unwrap().recv();

            // Executing the job is not included in the critical situation
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job to execute!");
                    job()
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
