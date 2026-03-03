use std::sync::Arc;
use io_uring::{IoUring, opcode};
use io_uring::types::Fd;
use super::session::IOSession;

pub(crate) trait IOOperation {
    fn request(&self, ring: IoUring);
    fn handle_completion(&self, result: i32);
}

pub(crate) struct IOOperationRecv<'a> {
    buffer: &'a mut [u8],
    session: Arc<IOSession>,
}

impl<'a> IOOperationRecv<'a> {
    fn new(session: Arc<IOSession>) -> Self {
        let buffer = session.get_buffer().as_mut_ptr();
        Self { buffer, session }
    }
}

impl<'a> IOOperation for IOOperationRecv<'a> {
    fn request(&self, ring: &IoUring) {
        let sq = ring.submission();

        unsafe {
            let sqe = opcode::Recv::new(
                self.session.get_fd(), 
                self.buffer.as_mut_ptr(), 
                self.buffer.len()
            ).build().user_data(self as *const Self as u64);

            sq.push(sqe);
        }
    }

    fn handle_completion(&self, result: i32) {
        self.session.on_received(result);
    }
}

pub(crate) struct IOOperationSend<'a> {
    buffer: &'a mut [u8],
    sessions: Vec<Arc<IOSession>>,
}

impl<'a> IOOperationSend<'a> {
    fn new(buffer: &'a mut [u8], sessions: Vec<Arc<IOSession>>) -> Self {
        Self { buffer, sessions }
    }
}

impl<'a> IOOperation for IOOperationSend<'a> {
    fn request(&self, ring: &IoUring) {
        let sq = ring.submission();

        unsafe {
            for session in self.sessions {
                let sqe = opcode::Send::new(
                    session.get_fd(), 
                    self.buffer.as_mut_ptr(), 
                    self.buffer.len()
                ).build().user_data(self as *const Self as u64);

                sq.push(sqe);
            }
        }
    }

    fn handle_completion(&self, result: i32) {
        for session in self.sessions {
            session.on_sent(result);
        }
    }
}

pub(crate) struct IOOperationAccept {
    listen_fd: Fd,
    session: Arc<IOSession>,
}

impl IOOperationAccept {
    pub fn new(listen_fd: Fd, session: Arc<IOSession>) -> Self {
        Self { listen_fd, session }
    }
}

impl IOOperation for IOOperationAccept {
    fn request(&self, ring: &IoUring) {
        let sq = ring.submission();

        unsafe {
            let sqe = opcode::Accept::new(
                self.listen_fd,
                self.session.get_fd(),
                0
            ).build().user_data(self as *const Self as u64);

            sq.push(sqe);
        }
    }

    fn handle_completion(&self, result: i32) {
        self.session.on_accepted(result);
    }
}

pub(crate) struct IOOperationClose {
    session: Arc<IOSession>,
}

impl IOOperationClose {
    pub fn new(session: Arc<IOSession>) -> Self {
        Self { session }
    }
}

impl IOOperation for IOOperationClose {
    fn request(&self, ring: &IoUring) {
        let sq = ring.submission();

        unsafe {
            let sqe = opcode::Close::new(
                self.session.get_fd()
            ).build().user_data(self as *const Self as u64);

            sq.push(sqe);
        }
    }

    fn handle_completion(&self, result: i32) {
        self.session.on_closed(result);
    }
}