use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => {
                println!("Connection closed by client");
                break;
            }
            Ok(n) => {
                if let Ok(message) = String::from_utf8(buffer[..n].to_vec()) {
                    println!("Received message: {}", message.trim());

                    let response = format!("Server received: {}", message);

                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprint!("Error writing to connection: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprint!("Error reading from connection: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprint!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
