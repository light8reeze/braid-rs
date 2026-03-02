use io_uring::types::Fd;

pub(crate) struct IOSession {
    session_fd: Fd,
    buffer: Vec<u8>,
    commited_size: i32,
}

impl IOSession {
    pub(crate) fn new(session_fd: Fd, buffer: Vec<u8>) -> Self {
        Self { session_fd, buffer }
    }

    pub(crate) fn get_fd(&self) -> Fd {
        self.session_fd
    }

    pub(crate) fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub(crate) fn handle_completion(result: i32) -> bool {
        if result > 0 {
            self.commited_size += result;
            //TODO: Handle packet event;
            return true;
        }

        false
    }
}