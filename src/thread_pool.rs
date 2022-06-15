use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct Pool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl Pool {
    pub fn new(capacity: usize) -> Pool {
        let mut workers = Vec::with_capacity(capacity);
        let (sender, receicer): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let r = Arc::new(Mutex::new(receicer));
        for _ in 0..capacity {
            workers.push(Worker::new(Arc::clone(&r)));
        }
        Pool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
        println!("Log:[Pool] Send new job to worker");
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Intr).unwrap();
            println!("Log:[Pool] Send intr to worker");
        }
    }
}

struct Worker {
    t: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let t = thread::spawn(move || {
            while let Ok(msg) = receiver.lock().unwrap().recv() {
                match msg {
                    Message::NewJob(job) => {
                        println!("Log:[Worker] Receive new job");
                        job();
                        println!("Log:[Worker] Done new job");
                    }
                    Message::Intr => {
                        println!("Log:[Worker] Receive intr, quit");
                        return;
                    }
                }
            }
        });
        Worker { t: Some(t) }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        match self.t.take() {
            Some(t) => t.join().unwrap(),
            _ => (),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Intr,
}
