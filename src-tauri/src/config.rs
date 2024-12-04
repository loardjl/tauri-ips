use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
// use std::path::PathBuf;
use std::env;
use std::path::{Path, PathBuf};

//配置文件结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub dc: DcConfig,
    pub cloud: CloudConfig,
    pub ips: IpsConfig,
    pub local: LocalConfig,
    pub show_logo: bool,
    pub eoms: EomsConfig,
    pub monitor: MonitorConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DcConfig {
    pub http: HttpObj,
    pub tcp: TcpObj,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudConfig {
    pub mcm: HostPortObj,
    pub iedp: HostPortObj,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpsConfig {
    pub tcp: HostPortObj,
    pub http: HostPortObj,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalConfig {
    pub http: PortObj,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EomsConfig {
    pub http: HostPortObj,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonitorConfig {
    pub enable: bool,
    pub mqtt: MqttObj,
    pub dir: DirObj,
    pub interval: IntervalObj,
    pub env: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpObj {
    pub host: String,
    pub port: u16,
    pub port_decision: Option<u16>,
    pub url: Option<String>,
    pub pre_url: Option<String>,
    pub decision_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TcpObj {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostPortObj {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortObj {
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MqttObj {
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirObj {
    pub win_error: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntervalObj {
    pub win_orrer: u64,
    pub services: u64,
    pub all_lathe: u64,
    pub device: u64,
    pub alarm: u64,
}

/**
 * 读取配置文件
 */
// pub fn app_config() -> Result<String, std::io::Error> {
//     let config_path = Path::new("config/config.yml");
//     let file_str = match std::fs::read_to_string(&config_path) {
//         Ok(file_str) => file_str,
//         Err(e) => return Err(e),
//     };
//     let app_config_obj: AppConfig = serde_yaml::from_str(&file_str).unwrap();
//     let app_config_str = serde_json::to_string(&app_config_obj).unwrap();
//     Ok(app_config_str)
// }
//读取配置文件
pub fn read_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let config_path = Path::new("config/config.yml");
    // // 打印 config_path
    //
    let real_path = exe_path.parent().unwrap().join(config_path);

    // let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // config_path.push("config/config.yml");
    println!("Config path: {:?}", real_path);
    let config_content = fs::read_to_string(real_path)?;
    let config: AppConfig = serde_yaml::from_str(&config_content)?;
    Ok(config)
}
