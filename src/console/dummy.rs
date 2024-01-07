// dummy.rs
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::thread;

use super::{Console, ConsoleCapable};

pub struct DummyConsole {
    pub console: Console,
}

impl DummyConsole {
    pub fn start(&self) {
        self.start_console_port(&self.console);
        self.start_client_handler(&self.console);
    }
}

impl ConsoleCapable for DummyConsole {

    fn handle_client(mut stream: UnixStream, name: String) {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    let received_data = &buffer[..n];
                    let received_str = String::from_utf8_lossy(received_data);
                    log::info!(
                        "Received on dummy console {} : {}",
                        name,
                        received_str.trim_end()
                    );

                    let write_back = format!("you said: {}", received_str);
                    stream.write_all(write_back.as_bytes()).unwrap();
                }
                Err(err) => {
                    log::error!("Error reading from socket on dummy console {} : {}", name, err);
                    break;
                }
            }
        }
    }

    fn start_console_port(&self, console: &Console) {
        let console_name = console.name.clone();
        thread::spawn(move || {
            log::info!("Start console port for {:?}", console_name);
        });
    }
}
