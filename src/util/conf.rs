use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Admin {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub name: String,
    // 日志等级
    pub log_level: String,
    pub v4port: u16,
    pub v6port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let file_path = "config.yml";
        let config = ServerConfig {
            name: "联合公社".to_string(), // 服务器名称
            log_level: "debug".to_string(), // 日志等级
            v4port: 2024,
            v6port: 2024,
        };
        match read_yml(&file_path) {
            Ok(config) => config,
            Err(_err) => {
                write_config_to_yml(&config, file_path).err();
                read_yml(&file_path).unwrap()
            }
        }
    }
}

// 写入到yml文件
pub fn write_config_to_yml(
    config: &ServerConfig,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_string = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(yaml_string.as_bytes())?;
    Ok(())
}

pub fn read_yml(file_path: &str) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: ServerConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}

