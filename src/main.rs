

use std::thread;
use console_server::console::{Console, create_listener};
use console_server::config::{ServerConfig, load_config};

fn main() {
    unsafe { libc::umask(0o077) };
    let cfg: ServerConfig = load_config("example.toml");

    let mut console_list: Vec<Console> = Vec::new();
    
    println!("Dummy consoles:");
    println!("*****************************************************");
    for dummy_config in cfg.dummy {
        println!("Name: {}", dummy_config.name);
        println!("Users_RW: {:?}", dummy_config.users_rw);
        println!("Socket Path: {:?}", dummy_config.socket_path);
        println!("\nTry it, with:\nsocat - UNIX-CONNECT:{}", dummy_config.socket_path);
        println!();
        let cons = Console{ 
            name: dummy_config.name.to_string(), 
            socket_path: dummy_config.socket_path,
            users_rw: dummy_config.users_rw,
        };
        console_list.push(cons);
    }



    println!("Start listeners:");
    println!("*****************************************************");
    for cons  in console_list {
        println!("{} : {}",cons.name, cons.socket_path);
        thread::spawn(move || {
            create_listener(cons);
        });
    }
    thread::park();
}

