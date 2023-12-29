
use std::thread;
use console_server::console::create_listener;
use console_server::config::{ServerConfig, load_config};

fn main() {
    env_logger::Builder::new()
    .filter_level(log::LevelFilter::Info)
    .format_timestamp_secs()
    .init();

    unsafe { libc::umask(0o077) };
    let cfg: ServerConfig = load_config("example.toml");

    for dummy_config in cfg.dummy {
        log::info!("Dummy Console {} for ", dummy_config.name);
        println!("Users_RW: {:?}", dummy_config.users_rw);
        println!("Socket Path: {:?}", dummy_config.socket_path);
        println!("\nTry it, with:\nsocat - UNIX-CONNECT:{}", dummy_config.socket_path);
        println!();
        thread::spawn(move || {
            create_listener(&dummy_config);
        });

    }


    thread::park();
}

