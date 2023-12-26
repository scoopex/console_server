// console.rs

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

#[derive(Clone, Debug, Default)]
pub struct Console { 
    pub name: String, 
    pub socket_path: String,
    pub users_rw: Vec<String>,
}

fn handle_client(mut stream: UnixStream, name: String) {
    // Handle incoming data from the client
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let received_data = &buffer[..n];
                let received_str = String::from_utf8_lossy(received_data);
                println!("Received on {} : {}", name, received_str.trim_end());

                let write_back = format!("you said: {}", received_str);
                stream.write_all(write_back.as_bytes()).unwrap();
                //stream.write_all(received_data).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}

pub fn create_listener(console: Console) {

    println!("Start server {} listening on {:?}", &console.name, &console.socket_path);
    std::fs::remove_file(&console.socket_path).ok();

    let listener = UnixListener::bind(&console.socket_path).expect("Failed to bind to socket");
    client_handler(listener, console);
}

fn client_handler(listener: UnixListener, console: Console) {
    
    println!("Start client handler for {:?}", console.name);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let console_name= console.name.clone();
                eprintln!("Accepting connection for {}", console_name);
                thread::spawn(move || {
                    handle_client(stream, console_name);
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection for {} : {}", console.name, err);
                break;
            }
        }
    }
}

