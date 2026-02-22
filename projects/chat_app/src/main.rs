//! # Simple Chat Server
//! 
//! แชทเซิร์ฟเวอร์แบบง่าย ใช้ TCP

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<Vec<TcpStream>>>;

fn handle_client(stream: TcpStream, clients: Clients) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    
    {
        let mut clients = clients.lock().unwrap();
        clients.push(stream.try_clone().unwrap());
    }
    
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let msg = line.clone();
                let clients = clients.lock().unwrap();
                for mut client in clients.iter() {
                    let _ = client.write_all(msg.as_bytes());
                }
            }
            Err(_) => break,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    
    println!("Chat server running on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
