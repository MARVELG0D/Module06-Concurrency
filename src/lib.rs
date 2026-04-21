use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Job adalah alias untuk closure yang akan dikirim ke thread
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Membuat ThreadPool baru.
    ///
    /// Ukuran (size) adalah jumlah thread di dalam pool.
    ///
    /// # Panics
    ///
    /// Fungsi `new` akan panic jika size bernilai 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Menggunakan Arc dan Mutex agar receiver bisa dibagikan ke banyak worker dengan aman
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Worker akan terus menunggu (block) sampai ada job yang diterima
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });

        Worker { id, thread }
    }
}