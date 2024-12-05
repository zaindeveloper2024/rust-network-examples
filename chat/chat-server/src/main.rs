use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientMap = Arc<Mutex<HashMap<usize, TcpStream>>>;

fn handle_client(mut stream: TcpStream, id: usize, clients: ClientMap) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();

    let welcome =  format!("Welcome to the chat server #{}\n", id);
    stream.write_all(welcome.as_bytes()).unwrap();
}

fn broadcast_message() {
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let mut client_id = 0;

    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
            println!("New client connected #{}", client_id);

            let clients = Arc::clone(&clients);
            clients.lock().unwrap().insert(client_id, stream.try_clone()?);

            thread::spawn(move || {
                handle_client(stream, client_id, clients);
            });

            client_id += 1;
        }
        Err(e) => {
            eprintln!("Error accepting client: {}", e);
        }
      }
    }

    Ok(())
}
