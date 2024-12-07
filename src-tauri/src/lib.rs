mod api_server;
mod config;
mod front_data_type;
mod msg_type;
mod nc_signal;
mod tcp_client;
mod utils;

use api_server::start_server;
use msg_type::ProtocolHeader;
use reqwest::header::{self, HeaderMap};
use serde_json::{json, Value};
use std::{result, sync::Arc};
use tauri::{command, Manager};
use tcp_client::{TcpClientManager, TcpConfig};
use tokio::sync::Mutex;

#[command]
async fn send_http_post_msg(url: String, data: Value) -> Result<String, ()> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    println!("url: {}, data: {:?}", url, data);
    let res = client
        .post(&url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();
    let body = res.text().await.unwrap();
    println!("res: {:?}", &body);
    Ok(body)
}
#[command]
async fn send_http_get_msg(url: String) -> Result<String, ()> {
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.unwrap();
    let body = res.text().await.unwrap();
    Ok(body)
}
#[command]
async fn get_api_host() -> Result<String, ()> {
    let config = config::read_config().expect("failed to read config");
    Ok(format!(
        "http://{}:{}",
        config.local.http.host, config.local.http.port
    ))
}

#[command]
async fn change_role(data: Value) -> Result<String, ()> {
    let result = api_server::change_role(data).await;
    match result {
        Ok(data) => Ok(data),
        Err(_) => Ok("failed".to_string()),
    }
}
#[command]
async fn get_role() -> String {
    api_server::get_role().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // 获取配置数据
    let config = config::read_config().expect("failed to read config");
    println!("{:?}", config);

    // 创建TCP客户端配置
    let client_config = TcpConfig {
        host: config.dc.tcp.host.clone(),
        port: config.dc.tcp.port,
    };
    let dc_client_config = TcpConfig {
        host: config.ips.tcp.host.clone(),
        port: config.ips.tcp.port,
    };

    // 创建TcpClientManager
    let manager = Arc::new(Mutex::new(TcpClientManager::new()));

    // 使用异步任务启动 TCP 客户端连接，避免阻塞主线程
    let manager_clone = manager.clone();
    let client_config_clone = client_config.clone();

    tokio::spawn(async move {
        let hearbeat = ProtocolHeader {
            header: 0x90eb,
            version: 0x01,
            order1: 0x0000,
            order2: 0x0200,
            state: 0x01,
            reset: 0x00000000,
            vor: 0x00,
            len: 0x0000,
        };
        if let Err(e) = manager_clone
            .lock()
            .await
            .add_client("dc".to_string(), client_config_clone, hearbeat)
            .await
        {
            eprintln!("Failed to connect dc TCP client: {}", e);
        } else {
            println!("dc TCP client connected successfully");
        }
    });
    // 使用异步任务启动 TCP 客户端连接，避免阻塞主线程
    let manager_clone_ips = manager.clone();
    let client_config_ips = dc_client_config.clone();

    tokio::spawn(async move {
        let hearbeat = ProtocolHeader {
            header: 0x90eb,
            version: 0x01,
            order1: 0x1600,
            order2: 0x0200,
            state: 0x01,
            reset: 0x00000000,
            vor: 0x00,
            len: 0x0000,
        };
        if let Err(e) = manager_clone_ips
            .lock()
            .await
            .add_client("ips".to_string(), client_config_ips, hearbeat)
            .await
        {
            eprintln!("Failed to connect ips TCP client: {}", e);
        } else {
            println!("ips TCP client connected successfully");
        }
    });

    // 启动 服务
    let manager_clone = manager.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(config.socket_server.tcp.port, manager_clone).await {
            eprintln!("Failed to start server: {}", e);
        }
    });

    tauri::Builder::default()
        .manage(manager)
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_http_post_msg,
            send_http_get_msg,
            get_api_host,
            change_role,
            get_role
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
