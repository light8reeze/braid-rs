use braid::thread::ThreadManager;
use braid::net::IOSession;
use std::net::TcpListener;


pub struct Service {
    session_count: i32,
    listener_list: Vec<TcpListener>,
    session_list: Vec<IOSession>,

    thread_count: i32,
    io_entries: i32,
}

impl Service {
    pub fn new() -> Self {
        Self { 
            session_count: 0, 
            listener_list: Vec::new(), 
            session_list: Vec::new(), 
            thread_count: 4, 
            io_entries: 16 
        }
    }

    pub fn set_thread_count(&mut self, thread_count: i32) {
        self.thread_count = thread_count;
    }

    pub fn set_io_entries(&mut self, io_entries: i32) {
        self.io_entries = io_entries;
    }

    pub fn set_session_count(&mut self, session_count: i32) {
        self.session_count = session_count;
    }

    pub fn add_listener(&mut self, listener: TcpListener) {
        self.listener_list.push(listener);
    }

    pub fn initialize(&self) {
        ThreadManager::instance().create_instance(self.thread_count, self.io_entries);
        
        if self.listener_list.is_empty() {
            panic!("No listeners added");
        }

        if self.session_count == 0 {
            self.session_count = 100;
        }

        for i in 0..self.session_count {
            let session = IOSession::new();
            self.session_list.push(session);
        }

        let mut session_index = 0;
        let max_idx = self.session_list.len() / self.listener_list.len();
        for listener in &self.listener_list {
            for _ in 0..max_idx {
                self.session_list[session_index].request_accept(listener.as_raw_fd());
                session_index += 1;
            }
        }
    }

    pub fn run(&self) {
        ThreadManager::instance().spawn();
        ThreadManager::instance().join();
    }
}