use std::sync::Arc;
use io_uring::IOUring;

trait IOOperation {
    pub fn request(&self, ring: IOUring);
    pub fn handle_completion(&self, result: i32);
}

pub(crate) struct IOOperationRecv {
    buffer: &mut [u8],
    session: Arc<IOSession>,
}

impl IOOperationRecv {
    pub fn new(session: Arc<IOSession>) -> Self {
        let buffer = session.get_buffer().as_mut_ptr();
        Self { buffer, session }
    }
}

impl IOOperation for IOOperationRecv {
    fn request(&self, ring: IOUring) {
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
        self.session.handle_completion(result);
    }
}

pub(crate) struct IOOperationSend {
    buffer: &mut [u8],
    sessions: Vec<Arc<IOSession>>,
}

impl IOOperationSend {
    pub fn new(buffer: &mut [u8], sessions: Vec<Arc<IOSession>>) -> Self {
        Self { buffer, sessions }
    }
}

impl IOOperation for IOOperationSend {
    fn request(&self, ring: IOUring) {
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
            session.handle_completion(result);
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
    fn request(&self, ring: IOUring) {
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
        self.session.handle_completion(result);
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
    fn request(&self, ring: IOUring) {
        let sq = ring.submission();

        unsafe {
            let sqe = opcode::Close::new(
                self.session.get_fd()
            ).build().user_data(self as *const Self as u64);

            sq.push(sqe);
        }
    }

    fn handle_completion(&self, result: i32) {
        self.session.handle_completion(result);
    }
}