use std::sync::mpsc::*;
use std::{rc, Duration};
use io_uring::{IOUring};
use io_uring::types::{SubmitArgs, Timespec}

trait IOOperation {
    pub fn request(&self, ring: IOUring);
    pub fn handle_completion(&self, result: i32);
}

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
    fn new(completed_operation: impl IOOperation) -> Self {
        Self { operation: completed_operation, }
    }

    fn set_result(&self, res: i32) {
        result = res;
    }

    fn get_result(&self) -> const i32 {
        result
    }

    fn handle_completion(&self) {
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

    pub(crate) fn flush(&self) {
        let mut iter = receiver.try_iter();
        while(let mut op = iter.next().is_some()) {
            op.request(ring);
        }
    }

    pub(crate) fn submit_and_wait(&self, timeout_ms: i32) -> IOCompletion {
        let ts = Timespec::from(Duration::from_millis(timeout_ms));
        let args = SubmitArgs::now().timespec(&ts);

        let _ = ring.submitter().submit_and_wait(1, &args);
        while(let Some(cqe) = ring.completion().next()) {
            // TODO: Completion handling
        }
    }
}