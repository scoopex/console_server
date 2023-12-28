// config.rs

use std::fs::File;
use std::io::Read;
use toml::Value;


pub struct GlobalConfig {
    pub socket_base_path: String,
    pub log_file: String,
    pub users_rw: Vec<String>,
}


pub struct SerialConfig {
    pub name: String,
    pub users_rw: Vec<String>,
    pub serial_device_name: String,
}

pub struct DummyConfig {
    pub name: String,
    pub users_rw: Vec<String>,
    pub socket_path: String,
}

pub struct ServerConfig {
    pub global: GlobalConfig,
    pub serial: Vec<SerialConfig>,
    pub dummy: Vec<DummyConfig>
}

pub fn load_config(file_name: &str) -> ServerConfig {
    // Read the TOML file into a String
    let mut toml_content = String::new();
    File::open(file_name)
        .and_then(|mut file| file.read_to_string(&mut toml_content))
        .expect("Failed to read TOML file");

    // Parse the TOML content
    let toml_value: Value = toml::from_str(&toml_content).expect("Failed to parse TOML content");

    let global_config = parse_global_config(&toml_value);
    let serial_config: Vec<SerialConfig> = parse_serial_configs(&toml_value);
    let dummy_config: Vec<DummyConfig> = parse_dummy_configs(&toml_value, &global_config.socket_base_path);

    // // Extract and process the values
    let result: ServerConfig  = ServerConfig {
        global: global_config,
        serial: serial_config,
        dummy: dummy_config,
    };
    return result;

    // println!("Serial Configs:");
    // for serial_config in &serial_configs {
    //     println!("Name: {}", serial_config.name);
    //     println!("Users_RW: {:?}", serial_config.users_rw);
    //     println!("Serial Device Name: {}", serial_config.serial_device_name);
    //     println!();
    // }

}


fn parse_global_config(toml_value: &Value) -> GlobalConfig {
  let global_section = toml_value["global"].as_table().unwrap();
  GlobalConfig {
      socket_base_path: global_section
          .get("socket_base_path")
          .and_then(Value::as_str)
          .unwrap_or("/tmp/example_socket")
          .to_string(),
      log_file: global_section
          .get("log_file")
         .and_then(Value::as_str)
         .unwrap_or("/console_server.log")
         .to_string(),
      users_rw: global_section
          .get("users_rw")
          .and_then(Value::as_array)
          .unwrap()
          .iter()
          .filter_map(|v| v.as_str().map(String::from))
          .collect(),
  }
}

fn parse_serial_configs(toml_value: &Value) -> Vec<SerialConfig> {
    if let Some(serial_array) = toml_value.get("serial_console").and_then(Value::as_array) {
        return serial_array
            .iter()
            .map(|serial_section| SerialConfig {
                name: serial_section["name"].as_str().unwrap_or_default().to_string(),
                users_rw: serial_section
                    .get("users_rw")
                    .and_then(Value::as_array)
                    .unwrap()
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect(),
                serial_device_name: serial_section
                    .get("serial_device_name")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            })
            .collect();
    }

    return Vec::new()
}

fn parse_dummy_configs(toml_value: &Value, socket_base_path: &String) -> Vec<DummyConfig> {
    if let Some(dummy_array) = toml_value.get("dummy_console").and_then(Value::as_array) {
        return dummy_array
            .iter()
            .map(|dummy_section| DummyConfig {
                name: dummy_section["name"].as_str().unwrap_or_default().to_string(),
                socket_path: format!(
                    "{}_{}",
                    socket_base_path,
                    dummy_section["name"].as_str().unwrap_or_default().to_string()
                ),
                users_rw: dummy_section
                    .get("users_rw")
                    .and_then(Value::as_array)
                    .unwrap()
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect(),
            })
            .collect();
    }

    return Vec::new();
}