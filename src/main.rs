use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    stream.write_all(b"connected!")?;

    let mut buffer = [0; 10];

    let n = stream.read(&mut buffer[..])?;

    println!("The bytes: {:?}", &buffer[..n]);

    stream.write(&buffer[..])?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4832")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { }
        }
    }

    Ok(())
}
