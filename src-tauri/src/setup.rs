use crate::api_server::start_server;
use crate::config;
use crate::msg_type::ProtocolHeader;
use crate::tcp_client::{TcpClientManager, TcpConfig};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

pub async fn set_up(handle: AppHandle) {
    let handle_clone = handle.clone();
    // 获取配置数据
    let config = config::read_config(&handle_clone)
        .await
        .expect("failed to read config");
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
}
