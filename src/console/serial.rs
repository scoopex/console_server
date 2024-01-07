// serial.rs

use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

use super::{Console, ConsoleCapable};

pub struct SerialConsole {
    pub console: Console,
}

impl SerialConsole {
    pub fn start(&self) {
        self.start_client_handler(&self.console);
    }
}
impl ConsoleCapable for SerialConsole {
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
                        "Received on serial console {} : {}",
                        name,
                        received_str.trim_end()
                    );

                    let write_back = format!("you said: {}", received_str);
                    stream.write_all(write_back.as_bytes()).unwrap();
                }
                Err(err) => {
                    log::error!("Error reading from socket on serial console {} : {}", name, err);
                    break;
                }
            }
        }
    }

    fn start_console_port(&self, console: &Console) {
        log::info!("Start console port for {:?}", console.name);
    }
}
