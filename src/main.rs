
use console_server::config::{load_config, ServerConfig};
use console_server::console::{Console, DummyConsole, SerialConsole};
use std::thread;
use clap::Parser;
use env_logger::Env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The configuration of the console server
    #[arg(short, long, default_value = "example.toml")]
    config: String,

    /// The log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}


fn main() {

    unsafe { libc::umask(0o077) };
    let args = Args::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();

    let cfg: ServerConfig = load_config(&args.config);

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
