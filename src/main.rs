
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
        thread::spawn(move || {
            create_listener(&dummy_config);
        });
    }
    // for serial_config in cfg.serial {
    //     thread::spawn(move || {
    //         create_listener(&serial_config);
    //     });
    // }


    thread::park();
}

