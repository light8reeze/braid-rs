
trait IOOperation {
    pub fn request(&self, ring: IOUring);
    pub fn handle_completion(&self, result: i32);
}

struct IOOperationRecv {
    buffer: &[u8],
}