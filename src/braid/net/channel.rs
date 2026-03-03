use std::sync::mpsc::*;
use std::rc::Rc;
use std::time::Duration;
use io_uring::IoUring;
use io_uring::types::{SubmitArgs, Timespec};
use braid::net::IOOperation;



pub(crate) struct IOCompletion {
    operation: Option<Box<dyn IOOperation>>,
    result: i32,
}

impl IOCompletion {
    pub(crate) fn new(completed_operation: impl IOOperation, res: i32) -> Self {
        Self { 
            operation: Some(Box::new(completed_operation)), 
            result: res,
        }
    }

    pub(crate) fn get_result(&self) -> i32 {
        self.result
    }

    pub(crate) fn handle_completion(&self) {
        self.operation?.handle_completion(self.result);
    }
}

pub(crate) struct IOChannel {
    sender: Sender<Box<dyn IOOperation>>,
    receiver: Receiver<Box<dyn IOOperation>>,
    ring : IoUring
}

impl IOChannel {
    pub(crate) fn new(&self, entries: i32) -> IOChannel {
        (self.sender, self.receiver) = channel();
        self.ring = IoUring::new(entries)?;
    }

    pub(crate) fn request(&self, operation: impl IOOperation) {
        self.sender.send(operation).unwrap();
    }

    pub(crate) fn flush_requests(&self) {
        let mut iter = self.receiver.try_iter();
        while let Some(op) = iter.next() {
            op.request(self.ring);
        }
    }

    pub(crate) fn submit_and_wait(&self, timeout_ms: i32) -> Option<IOCompletion> {
        let ts = Timespec::from(Duration::from_millis(timeout_ms));
        let args = SubmitArgs::now().timespec(&ts);

        let _ = self.ring.submitter().submit_and_wait(1, &args);
        if let Some(cqe) = self.ring.completion().next() {
            let user_data = cqe.user_data();

            let ptr = user_data as *mut dyn IOOperation;

            unsafe {
                let op = Box::from_raw(ptr);
                
                Some(IOCompletion::new(&op, cqe.result()))
            }
        }

        None
    }

    pub(crate) fn process(&self, timeout_ms: i32) {
        self.flush_requests();

        if let Some(completion) = self.submit_and_wait(timeout_ms){
            completion.handle_completion();
            self.ring.comletion().sync();
        }
    }
}