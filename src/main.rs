use std::net::TcpListener;

pub mod braid;

fn main() {
    let service = braid::net::Service::new();
    let listener = TcpListener::bind("127.0.0.1:4832").unwrap();
    service.add_listener(listener);
    service.initialize();
    service.run();
}