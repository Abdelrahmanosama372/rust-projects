use std::sync::{mpsc::{self, channel}, Arc, Mutex};


pub struct Threadpoll {
    sender: mpsc::Sender<job>,
    workers: Vec<worker>
}

struct worker {
    id: u32,
    join_handle: std::thread::JoinHandle<()>,
}

type job = Box<dyn FnOnce() + Send + 'static>;

impl Threadpoll {
    pub fn new(size: usize) -> Threadpoll {
        assert!(size > 0);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        //let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(worker::new(i as u32,receiver.clone()));
        }
        Threadpoll {
            sender: sender,
            workers: workers
        }
    }

    pub fn execute<F>(&self, closure: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);
       self.sender.send(job).unwrap();
    }
}

impl worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<job>>>) -> worker {
        let join_handle = std::thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv(); 
                match message {
                    Ok(job) => {
                        println!("thread {id} executing the job");
                        job();
                    }
                    Err(_) => println!("thread {id} error in transfered message")
                }
            }
        );
        worker {id:id,join_handle:join_handle}
    }
}