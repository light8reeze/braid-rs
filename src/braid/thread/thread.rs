use std::thread;
use std::sync::atomic::AtomicUsize;
use braid::net::IOChannel;
use braid::net::IOOperation;

pub struct WorkerThread {
    isStop: bool,
    thread: Option<thread::JoinHandle<()>>,
    io_channel: IOChannel,
    io_entries: i32,
}

trait EventThread {
    fn initialize(&self);
    fn routine(&self) -> bool;
    fn spawn(&self);
    fn join(&self);
    fn stop(&self);
}

impl WorkerThread {
    pub(crate) fn new(io_entries: i32) -> Self {
        Self {
            isStop: false,
            thread: None,
            io_channel: IOChannel::new(io_entries),
            io_entries,
        }
    }

    pub fn request(&self, operation: impl IOOperation) {
        self.io_channel.request(operation);
    }
}

impl EventThread for WorkerThread {
    fn initialize(&self) {
    }

    fn routine(&self) -> bool {
        self.io_channel.flush_requests();

        if let Some(completion) = self.io_channel.submit_and_wait(1) {
            completion.handle_completion();
        }

        true
    }

    fn spawn(&self) {
        self.thread = Some(thread::spawn(move || {
            while !self.isStop {
                self.routine();
            }
        }));
    }

    fn join(&self) {
        self.thread.as_ref().unwrap().join().unwrap();
    }

    fn stop(&self) {
        self.isStop = true;
    }
}

pub(crate) struct ThreadManager {
    threads: Vec<Box<dyn EventThread>>,
    io_index: AtomicUsize,

    instance: Option<ThreadManager>,
}

impl ThreadManager {
    pub(crate) fn new(thread_count: i32, io_entries: i32) -> Self {
        Self {
            threads: (0..thread_count).map(|_| Box::new(WorkerThread::new(io_entries))).collect(),
            io_index: AtomicUsize::new(0),
            instance: None,
        }
    }

    pub(crate) fn request(&self, operation: impl IOOperation) {
        let index = self.io_index.fetch_add(1) % self.threads.len();
        let thread = &self.threads[index];
        thread.request(operation);
    }

    pub(crate) fn spawn(&self) {
        for thread in &self.threads {
            thread.spawn();
        }
    }

    pub(crate) fn join(&self) {
        for thread in &self.threads {
            thread.join();
        }
    }

    pub(crate) fn stop(&self) {
        for thread in &self.threads {
            thread.stop();
        }
    }

    pub fn initialize(&self) {
        for thread in &self.threads {
            thread.initialize();
        }
    }

    pub fn create_instance(thread_count: i32, io_entries: i32) -> Self {
        ThreadManager::new(thread_count, io_entries)
    }

    pub fn instance(&mut self) -> &'static mut ThreadManager {
        if self.instance.is_none() {
            self.instance = Some(ThreadManager::create_instance(4, 16));
        }

        self.instance.as_mut().unwrap()
    }
}

impl Drop for ThreadManager {
    fn drop(&mut self) {
        self.stop();
        self.join();
    }
}