// dummy.rs
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::thread;
use bus::Bus;
use std::sync::{Arc,Mutex};
use std::time::Duration;
use super::{Console, ConsoleActivity, ConsoleCapable};

pub struct DummyConsole {
    pub console: Console,
}

impl DummyConsole {
    pub fn start(&self) {
        let bus: Bus<ConsoleActivity> = Bus::new(10);
        let protected_bus = Arc::new(Mutex::new(bus));
        self.start_console_port(&self.console, protected_bus.clone());
        self.start_client_handler(&self.console, protected_bus);
        //self.start_client_handler(&self.console, protected_bus.clone());
    }
}

impl ConsoleCapable for DummyConsole {

    fn handle_client(mut stream: UnixStream, name: String, arc_bus_client: Arc<Mutex<Bus<ConsoleActivity>>>) {
        let mut count = 0;
        let mut buffer = [0; 1024];

        let mut rx1 = arc_bus_client.lock().unwrap().add_rx();
        log::debug!("Added a receiver");
        loop {
            let event = rx1.recv().unwrap();
            let write_back = format!("recv thread {:?}: >>>{}<<<, {} values received\n", thread::current().id(), event.body, count);
            stream.write_all(write_back.as_bytes()).unwrap();
            count += 1;
        }
        // loop {
        //     match stream.read(&mut buffer) {
        //         Ok(n) => {
        //             if n == 0 {
        //                 break;
        //             }
        //             let received_data = &buffer[..n];
        //             let received_str = String::from_utf8_lossy(received_data);
        //             log::info!(
        //                 "Received on dummy console {} : {}",
        //                 name,
        //                 received_str.trim_end()
        //             );
        //
        //             let write_back = format!("you said: {}", received_str);
        //             stream.write_all(write_back.as_bytes()).unwrap();
        //         }
        //         Err(err) => {
        //             log::error!("Error reading from socket on dummy console {} : {}", name, err);
        //             break;
        //         }
        //     }
        // }
    }

    fn start_console_port(&self, console: &Console, arc_bus: Arc<Mutex<Bus<ConsoleActivity>>>) {
        let console_name = console.name.clone();
        thread::spawn(move || {
            let mut count = 0;
            loop {
                let event = ConsoleActivity{
                    body: format!("Message on console {} ~ This is message number {}", console_name, count),
                };
                log::debug!("Sending : {}", event.body);
                {
                    let mut bus = arc_bus.lock().unwrap();
                    bus.broadcast(event);
                }
                count += 1;
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}
