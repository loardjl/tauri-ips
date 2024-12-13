use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, str};
use tokio::sync::Mutex;
// use std::path::PathBuf;
use lazy_static::lazy_static;
use tauri::AppHandle;

//配置文件结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub dc: DcConfig,
    pub cloud: CloudConfig,
    pub ips: IpsConfig,
    pub socket_server: SocketServerObj,
    pub local: LocalConfig,
    pub show_logo: bool,
    pub eoms: EomsConfig,
    pub monitor: MonitorConfig,
}
impl AppConfig {
    fn new() -> AppConfig {
        AppConfig {
            dc: DcConfig {
                http: HttpObj {
                    host: "".to_string(),
                    port: 0,
                    port_decision: None,
                    url: None,
                    pre_url: None,
                    decision_url: None,
                },
                tcp: TcpObj {
                    host: "".to_string(),
                    port: 0,
                },
            },
            cloud: CloudConfig {
                mcm: HostPortObj {
                    host: "".to_string(),
                    port: 0,
                },
                iedp: HostPortObj {
                    host: "".to_string(),
                    port: 0,
                },
            },
            ips: IpsConfig {
                tcp: HostPortObj {
                    host: "".to_string(),
                    port: 0,
                },
                http: HostPortObj {
                    host: "".to_string(),
                    port: 0,
                },
            },
            socket_server: SocketServerObj {
                tcp: PortObj { port: 0 },
            },
            local: LocalConfig {
                http: HostPortObj {
                    host: "".to_string(),
                    port: 0,
                },
            },
            show_logo: false,
            eoms: EomsConfig {
                http: HostPortObj {
                    host: "".to_string(),
                    port: 0,
                },
            },
            monitor: MonitorConfig {
                enable: false,
                mqtt: MqttObj {
                    host: "".to_string(),
                },
                dir: DirObj {
                    win_error: "".to_string(),
                },
                interval: IntervalObj {
                    win_orrer: 0,
                    services: 0,
                    all_lathe: 0,
                    device: 0,
                    alarm: 0,
                },
                env: "".to_string(),
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocketServerObj {
    pub tcp: PortObj,
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
    pub http: HostPortObj,
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

lazy_static! {
    pub static ref APP_CONFIG: Mutex<AppConfig> = Mutex::new(AppConfig::new());
}

//读取配置文件
pub async fn read_config(handle: &AppHandle) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = handle
        .path_resolver()
        .resolve_resource("config/config.yml")
        .expect("failed to resolve resource");
    let config_content = fs::read_to_string(config_path)?;
    let config: AppConfig = serde_yaml::from_str(&config_content)?;
    // 设置全局配置
    let mut app_config = APP_CONFIG.lock().await;
    app_config.clone_from(&config);
    println!("Config: {:?}", app_config);
    Ok(config)
}

pub async fn get_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let app_config = APP_CONFIG.lock().await;
    Ok(app_config.clone())
}
