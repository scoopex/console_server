use console_server::config::{load_config, ServerConfig};
use console_server::console::{Console, DummyConsole, SerialConsole};
use std::thread;

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .init();

    unsafe { libc::umask(0o077) };
    let cfg: ServerConfig = load_config("example.toml");

    for dummy_config in cfg.dummy {
        let socket_base_path = cfg.global.socket_base_path.clone();
        thread::spawn(move || {
            DummyConsole {
                console: Console::new(
                    dummy_config.name,
                    dummy_config.users_rw,
                    dummy_config.users_ro,
                    socket_base_path,
                ),
            }.start();
        });
    }

    for serial_config in cfg.serial {
        let socket_base_path = cfg.global.socket_base_path.clone();
        thread::spawn(move || {
            SerialConsole {
                console: Console::new(
                    serial_config.name,
                    serial_config.users_rw,
                    serial_config.users_ro,
                    socket_base_path,
                ),
            }.start();
        });
    }


    thread::park();
}
