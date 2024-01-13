// console.rs

pub mod serial;
pub mod dummy;

use exacl::{setfacl, AclEntry, Perm};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};

use std::thread;
use bus::Bus;

pub struct Console {
    pub name: String,
    pub users_rw: Vec<String>,
    pub users_ro: Vec<String>,
    pub socket_path: String,
}

#[derive(Clone)]
pub struct ConsoleActivity {
    pub body: String,
}

impl Console {
    pub fn new(
        name: String,
        users_rw: Vec<String>,
        users_ro: Vec<String>,
        socket_base_path: String,
    ) -> Console {
        Console {
            name: name.clone(),
            users_ro: users_ro,
            users_rw: users_rw,
            socket_path: format!("{}_{}", socket_base_path, name.clone()),
        }
    }

    fn set_socket_permissions(&self) {
        let mut entries: Vec<AclEntry> = vec![];

        entries.push(AclEntry::allow_user("", Perm::READ | Perm::WRITE, None));
        entries.push(AclEntry::allow_group("", Perm::empty(), None));
        entries.push(AclEntry::allow_other(Perm::empty(), None));
        entries.push(AclEntry::allow_mask(Perm::empty(), None));
        log::debug!("{} - Users_RW: {:?}", self.name, self.users_rw);
        for username in &self.users_rw {
            entries.push(AclEntry::allow_user( username, Perm::READ | Perm::WRITE, None,));
        }
        log::debug!("{} - Users_RO: {:?}", self.name, self.users_ro);
        for username in &self.users_ro {
            entries.push(AclEntry::allow_user(username, Perm::READ, None));
        }

        match setfacl(&[&self.socket_path], &entries, None) {
            Ok(_) => log::info!(
                "ACL file permissions successfully set for {}",
                &self.socket_path
            ),
            Err(e) => log::error!(
                "Error setting file permissions for {}: {}",
                &self.socket_path,
                e
            ),
        }
    }

    pub fn get_listener(&self) -> UnixListener {
        log::info!(
            "Start server {} listening on {:?}",
            &self.name,
            &self.socket_path
        );
        log::debug!("Try it, with:\n\nsocat - UNIX-CONNECT:{}\n\n", self.socket_path);

        std::fs::remove_file(&self.socket_path).ok();

        let listener = UnixListener::bind(&self.socket_path).expect("Failed to bind to socket");
        self.set_socket_permissions();
        return listener;
    }
}

pub trait ConsoleCapable {
    fn handle_client(stream: UnixStream, name: String, arc_bus_client: Arc<Mutex<Bus<ConsoleActivity>>>);

    fn start_console_port(&self, console: &Console,  arc_bus: Arc<Mutex<Bus<ConsoleActivity>>>);

    fn start_client_handler(&self, console: &Console, arc_bus: Arc<Mutex<Bus<ConsoleActivity>>>) {
        log::info!("Start client handler for {:?}", console.name);

        let listener = console.get_listener();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    log::info!("Accepting connection for {}", console.name);
                    let console_name = console.name.clone();
                    let arc_bus_client = arc_bus.clone();
                    thread::spawn(move || {
                        Self::handle_client(stream, console_name, arc_bus_client);
                    });
                }
                Err(err) => {
                    log::error!("Error accepting connection for {} : {}", console.name, err);
                    break;
                }
            }
        }
    }
}