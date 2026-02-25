use std::sync::mpsc::*;
use io_uring;
use std::rc;

trait IOOperation {
    pub fn request(&self, ring: IOUring);
    pub fn handle_completion(&self, result: i32);
}

pub(crate) struct IOChannel {
    sender: Sender<Rc<IOOperation>>,
    receiver: Recceiver<Rc<IOOperation>>,
    ring : IoUring,
};

impl IOChannel {
    pub(crate) fn new(&self) -> IOChannel {
        (self.sender, self.receiver) = Channel();
    }

    pub(crate) fn request(&self, operation: impl IOOperation) {

    }

    pub(crate) fn handle_completion(&self, result: i32) {
        
    }
}