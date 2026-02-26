use std::sync::mpsc::*;
use std::{rc, Duration};
use io_uring::{IOUring};
use io_uring::types::{SubmitArgs, Timespec}

pub(crate) struct IOChannel {
    sender: Sender<Rc<IOOperation>>,
    receiver: Recceiver<Rc<IOOperation>>,
    ring : IoUring,
};

pub(crate) struct IOCompletion {
    operation: Option<impl IOOperation>,
    result: i32,
}

impl IOCompletion {
    pub(crate) fn new(completed_operation: impl IOOperation, res: i32) -> Self {
        Self { 
            operation: completed_operation, 
            result: res,
        }
    }

    pub(crate) fn get_result(&self) -> const i32 {
        result
    }

    pub(crate) fn handle_completion(&self) {
        operation?.handle_completion(result);
    }
}

impl IOChannel {
    pub(crate) fn new(&self, entries: i32) -> IOChannel {
        (self.sender, self.receiver) = Channel();
        ring = IoUring::new(entries)?;
    }

    pub(crate) fn request(&self, operation: impl IOOperation) {
        sender.send(operation).unwrap();
    }

    pub(crate) fn flush_requests(&self) {
        let mut iter = receiver.try_iter();
        while(let mut some(op) = iter.next()) {
            op.request(ring);
        }
    }

    pub(crate) fn submit_and_wait(&self, timeout_ms: i32) -> Option<IOCompletion> {
        let ts = Timespec::from(Duration::from_millis(timeout_ms));
        let args = SubmitArgs::now().timespec(&ts);

        let _ = ring.submitter().submit_and_wait(1, &args);
        if let Some(cqe) = ring.completion().next() {
            let user_data = cqe.user_data();

            let ptr = user_data as *mut IOOperation;

            unsafe {
                let op = Box::from_raw(ptr);
                
                Some(IOCompletion::new(&op, cqe.result()))
            }
        }

        None
    }

    pub(crate) fn process(&self, timeout_ms: i32) {
        flush_requests();

        if let some(completion) = submit_and_wait(timeout_ms){
            completion?.handle_completion();
            ring.comletion().sync();
        }
    }
}