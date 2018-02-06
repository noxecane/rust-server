use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub struct Pool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

enum Message {
    Execute(Job),
    Quit
}

type Job = Box<FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}


impl Pool {
    /// Create new `Pool` of with `size` worker threads
    ///
    /// # Panics
    ///
    /// The `new` function will panic when the size == 0
    pub fn new(size: usize) -> Pool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Pool { workers, sender }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(task);
        self.sender.send(Message::Execute(job)).unwrap();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Quit).unwrap();
        }

        // in seperate loop to make sure all threads got their
        // Quit message
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let handle = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Execute(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    },
                    Message::Quit => {
                        println!("Terminating worker {}", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            handle: Some(handle),
        }
    }
}
