use io_uring::types::Fd;
use std::sync::Arc;
use braid::thread::ThreadManager;
use super::operation::{IOOperationRecv, IOOperationSend, IOOperationAccept, IOOperationClose};

pub(crate) struct IOSession {
    session_fd: Fd,
    buffer: Vec<u8>,
    commited_size: i32,
}

pub(crate) trait IORequest {
    fn request_recv(&self);
    fn request_send(&self, buffer: &[u8]);
    fn request_accept(&self);
    fn request_close(&self);
}

impl IORequest for IOSession {
    fn request_recv(&self) {
        ThreadManager::instance().request(IOOperationRecv::new(Arc::new(self)));
    }

    fn request_send(&self, buffer: &[u8]) {
        ThreadManager::instance().request(IOOperationSend::new(buffer, vec![Arc::new(self)]));
    }

    fn request_accept(&self, listen_fd: Fd) {
        ThreadManager::instance().request(IOOperationAccept::new(listen_fd, Arc::new(self)));
    }

    fn request_close(&self) {
        ThreadManager::instance().request(IOOperationClose::new(Arc::new(self)));
    }
}

pub(crate) trait IOEvent {
    fn on_received(&self, result: i32) -> bool;
    fn on_sent(&self, result: i32) -> bool;
    fn on_accepted(&self, result: i32) -> bool;
    fn on_closed(&self, result: i32) -> bool;
}

impl IOEvent for IOSession {
    fn on_received(&self, result: i32) -> bool {
        if result > 0 {
            self.commited_size += result;
            //TODO: Handle packet event;
            println!("Commited size: {}", self.commited_size);
            self.request_send(&self.buffer[0..self.commited_size as usize]);
            self.request_recv();

            return true;
        }

        false
    }

    fn on_sent(&self, result: i32) -> bool {
        if result > 0 {
            println!("Sent size: {}", result);
            return true;
        }

        false
    }

    fn on_accepted(&self, result: i32) -> bool {
        if result > 0 {
            self.request_recv();

            return true;
        }

        false
    }

    fn on_closed(&self, result: i32) -> bool {
        if result > 0 {
            return true;
        }

        false
    }
}

impl IOSession {
    fn new() -> Self {
        Self { session_fd: Fd(0), buffer: Vec::new(), commited_size: 0 }
    }

    fn set_fd(&mut self, session_fd: Fd) {
        self.session_fd = session_fd;
    }

    fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    fn get_fd(&self) -> Fd {
        self.session_fd
    }

    fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}