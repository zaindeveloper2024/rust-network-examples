use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => {
            println!("Successfully connected to server");
            stream
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Err(e);
        }
    };

    loop {
        let mut input = String::new();
        print!("Enter message (or 'quit' to exit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;


        if input.trim().to_lowercase() == "quit" {
            println!("Exiting...");
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Server closed connection");
                    break
                }
                let response = String::from_utf8_lossy(&buffer);
                println!("Received: {}", response);
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
                break;
            }
        }
    }

    Ok(())
}
