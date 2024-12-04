use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");

    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(n) if n == 0 => break,
                Ok(n) => {
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(_) => break,
            }
        }
    });

    let mut input = String::new();
    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        if stream.write_all(input.as_bytes()).is_err() {
            break;
        }
    }
}