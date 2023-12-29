// console.rs

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use exacl::{setfacl, AclEntry, Perm};

use crate::config::DummyConfig;

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
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}


fn set_file_permissions(console: &DummyConfig) {
    let mut entries: Vec<AclEntry> = vec![];

    entries.push(AclEntry::allow_user("", Perm::READ | Perm::WRITE, None));
    entries.push(AclEntry::allow_group("", Perm::empty(), None));
    entries.push(    AclEntry::allow_other(Perm::empty(), None));
    entries.push(    AclEntry::allow_mask(Perm::empty(), None));
    
    for username in &console.users_rw {
        entries.push(
            AclEntry::allow_user(username, Perm::READ | Perm::WRITE, None)
        );
    }

    match setfacl(&[&console.socket_path], &entries, None){
        Ok(_) => println!("ACL file permissions successfully set for {}", &console.socket_path),
        Err(e) => eprintln!("Error setting file permissions for {}: {}", &console.socket_path, e),
    }
}

pub fn create_listener(console: &DummyConfig) {

    println!("Start server {} listening on {:?}", &console.name, &console.socket_path);
    std::fs::remove_file(&console.socket_path).ok();

    let listener = UnixListener::bind(&console.socket_path).expect("Failed to bind to socket");
    set_file_permissions(console);
    client_handler(listener, console);
}

fn client_handler(listener: UnixListener, console: &DummyConfig) {
    
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

