use crate::msg_type::{GetToken, MsgType, OptimizeInfo, ProtocolHeader, RealTimeData};
use crate::nc_signal::{set_nc_signal_val, NcSignal, NcSignalManager, NcSignalVal};
use crate::tcp_client::TcpClientManager;

use axum::routing::get;
use bincode::serialize;
use lazy_static::lazy_static;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

type TcpClientManager1 = Arc<Mutex<TcpClientManager>>;

#[derive(Debug, Serialize, Deserialize)]
struct SendData {
    data: Vec<u8>,
    is_json: bool,
    msg: MsgType,
}
#[derive(Debug, Serialize, Deserialize)]
struct FrontData {
    data: Value,
    msg: String,
}

lazy_static! {
    static ref NC_SIGNAL_MANAGER: Mutex<NcSignalManager> = Mutex::new(NcSignalManager::new()); // 用于采集配置存储实时信号值
    static ref START_PUSH_DATA: Mutex<bool> = Mutex::new(false); // 用于控制是否推送信号值
    static ref ROLE: Mutex<String> = Mutex::new("".to_string()); // 用于存储当前角色
}

pub async fn change_role(data: Value) -> Result<String, ()> {
    let mut role = ROLE.lock().await;
    let cur_role = data["role"].as_str().unwrap();
    let password = data["password"].as_str().unwrap();
    if cur_role == "user" {
        *role = cur_role.to_string();
        Ok("success".to_string())
    } else if cur_role == "admin" && password == "admin" {
        *role = cur_role.to_string();
        Ok("success".to_string())
    } else {
        Ok("failed".to_string())
    }
}

pub async fn get_role() -> String {
    let role = ROLE.lock().await;
    role.clone()
}

pub async fn get_nc_signal_val(socket: &SocketRef) {
    let nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
    let signal_data = nc_signal_manager.realtime_data.get("dc").cloned();
    drop(nc_signal_manager);
    match signal_data {
        Some(signal_data) => {
            socket.emit("StartPushData", &json!(signal_data)).ok();
        }
        None => error!("signal_data is None"),
    }
}

async fn send_tcp(manager: TcpClientManager1, data: Vec<u8>, topic: &str) {
    let manager_clone = manager.clone();
    info!("Received message: {:?}", data);
    let manager = manager_clone.lock().await;
    if let Some(client) = manager.get_client(topic) {
        let client = client.clone();
        drop(manager); // 释放 manager 的锁
        info!("Sending data to TCP client: {:?}", data);
        {
            let client = client.lock().await; // 仅在需要操作 client 时加锁
            client.send(&data).await.expect("Failed to send data");
        }
    }
}

// 处理 TCP 消息
async fn handle_tcp_messages(client_name: &str, manager: TcpClientManager1, socket: SocketRef) {
    let client = {
        let manager = manager.lock().await;
        if let Some(client) = manager.get_client(client_name) {
            drop(manager); // 释放 manager 的锁
            client.clone()
        } else {
            error!("Client '{}' not found", client_name);
            return;
        }
    };
    let client = client.lock().await;
    let receiver = client.get_receiver();
    let topic = client.get_topic();
    drop(client);
    loop {
        let mut receiver_guard = receiver.lock().await;
        match receiver_guard.recv().await {
            Some(response) => {
                drop(receiver_guard); // 提前释放锁
                match serde_json::from_slice::<SendData>(&response) {
                    Ok(send_data) => {
                        let data = send_data.data;
                        let msg = send_data.msg;
                        let msg_str = MsgType::to_string(&msg);
                        if send_data.is_json {
                            match serde_json::from_slice::<Value>(&data) {
                                Ok(parsed_response) => {
                                    info!("Received response (JSON): {:?}", parsed_response);
                                    socket.emit(msg_str, &parsed_response).ok();
                                }
                                Err(e) => error!("Failed to parse JSON response: {}", e),
                            }
                        } else {
                            let decoded = decode_data(data, &msg, &topic).await;
                            info!("Received response: {:?}", decoded);
                            if msg == MsgType::NcSignalVal {
                                continue;
                            }
                            socket.emit(msg_str, &decoded).ok();
                        }
                    }
                    Err(e) => error!("Failed to deserialize SendData: {}", e),
                }
            }
            None => {
                error!("TCP client '{}' receiver closed.", client_name);
                break;
            }
        }
    }
}

// 监听 TCP 连接状态
async fn maintain_connection(client_name: &str, manager: TcpClientManager1) {
    let client = {
        let manager = manager.lock().await;
        if let Some(client) = manager.get_client(client_name) {
            drop(manager); // 释放 manager 的锁
            client.clone()
        } else {
            error!("Client '{}' not found", client_name);
            return;
        }
    };

    loop {
        let mut client = client.lock().await;
        client.watch_connection().await;
        drop(client);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
async fn handle_connec(
    socket: &SocketRef,
    manager: TcpClientManager1,
) -> Result<(), Box<dyn std::error::Error>> {
    let manager_clone = manager.clone();
    socket.on("IpsRegister", move |Data::<Value>(data)| {
        let manager_clone = manager_clone.clone();
        info!("Received IpsRegister message: {:?}", data);
        tokio::spawn(async move {
            let ips_register = ProtocolHeader {
                header: 0x90eb,
                version: 0x01,
                order1: 0x1600,
                order2: 0x0101,
                state: 0x00,
                reset: 0x00000000,
                vor: 0x00,
                len: 0x0000,
            };
            let serialized_data = serialize(&ips_register).unwrap();
            send_tcp(manager_clone, serialized_data, "ips").await;
        });
    });
    let manager_clone = manager.clone();
    socket.on("getToken", move |Data::<Value>(data)| {
        let manager_clone = manager_clone.clone();
        info!("Received getToken message: {:?}", data);
        tokio::spawn(async move {
            let ips_register = ProtocolHeader {
                header: 0x90eb,
                version: 0x01,
                order1: 0x0000,
                order2: 0x0101,
                state: 0x00,
                reset: 0x00000000,
                vor: 0x00,
                len: 0x0000,
            };
            let serialized_data = serialize(&ips_register).unwrap();
            send_tcp(manager_clone, serialized_data, "dc").await;
        });
    });
    let socket_clone = socket.clone();
    socket.on("startPushData", move |Data::<Value>(data)| {
        info!("Received startPushData message: {:?}", data);
        tokio::spawn(async move {
            let mut start_push_data = START_PUSH_DATA.lock().await;
            *start_push_data = true;
            drop(start_push_data);
            // 每隔 1 秒推送一次信号值
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let start_push_data = START_PUSH_DATA.lock().await;
                let should_push = *start_push_data;
                if should_push {
                    info!("should_push: {:?}", should_push);
                    get_nc_signal_val(&socket_clone).await;
                } else {
                    break;
                }
            }
        });
    });
    socket.on("stopPushData", move |Data::<Value>(data)| {
        tokio::spawn(async move {
            info!("Received stopPushData message: {:?}", data);
            let mut start_push_data = START_PUSH_DATA.lock().await;
            *start_push_data = false;
        });
    });
    // 启动 TCP 消息接收与连接状态维护
    tokio::spawn(handle_tcp_messages("dc", manager.clone(), socket.clone()));
    tokio::spawn(maintain_connection("dc", manager.clone()));
    tokio::spawn(handle_tcp_messages("ips", manager.clone(), socket.clone()));
    tokio::spawn(maintain_connection("ips", manager.clone()));

    Ok(())
}

async fn decode_data(data: Vec<u8>, msg: &MsgType, topic: &str) -> Result<Value, String> {
    match msg {
        MsgType::RealTimeData => {
            let result = RealTimeData::parse_real_time_data(&data);
            match result {
                Ok(value) => {
                    info!("Decoded RealTimeData: {:?}", value);
                    let value = json!(value);
                    Ok(value)
                }
                Err(e) => Err(format!("Failed to parse RealTimeData: {}", e)),
            }
        }
        MsgType::OptimizeInfo => {
            let result = OptimizeInfo::to_string(&data);
            match result {
                Ok(value) => {
                    info!("Decoded OptimizeInfo: {:?}", value);
                    let value = json!(value);
                    Ok(value)
                }
                Err(e) => Err(format!("Failed to parse OptimizeInfo: {}", e)),
            }
        }
        MsgType::NcSignalVal => {
            let signal_value = NcSignal::parse_nc_signal(&data).unwrap();
            if topic == "dc" {
                let nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
                let signal_data = nc_signal_manager.realtime_data.get("dc").cloned();
                drop(nc_signal_manager);
                match signal_data {
                    Some(mut signal_data) => {
                        set_nc_signal_val(&mut signal_data, signal_value).await;
                        let mut nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
                        nc_signal_manager
                            .realtime_data
                            .insert("dc".to_string(), signal_data);
                        drop(nc_signal_manager);
                    }
                    None => {
                        let nc_signal_val =
                            NcSignalVal::new(signal_value.dev_id, signal_value.collector_id);
                        let mut realtime_data = HashMap::new();
                        realtime_data.insert(signal_value.collector_id, nc_signal_val);
                        set_nc_signal_val(&mut realtime_data, signal_value).await;
                        let mut nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
                        nc_signal_manager
                            .realtime_data
                            .insert("dc".to_string(), realtime_data);
                        drop(nc_signal_manager);
                    }
                }
            }
            Ok(json!(""))
        }
        MsgType::GetToken => {
            let result = GetToken::parse_get_token(&data);
            match result {
                Ok(value) => {
                    info!("Decoded GetToken: {:?}", value);
                    let value = json!(value);
                    Ok(value)
                }
                Err(e) => Err(format!("Failed to parse GetToken: {}", e)),
            }
        }
        MsgType::Unknown => Err("Unknown message type".to_string()),
    }
}

/// 启动 HTTP 服务器
pub async fn start_server(
    port: u16,
    manager: TcpClientManager1,
) -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", move |socket: SocketRef| {
        let manager_clone = manager.clone();
        let socket_clone = socket.clone();
        async move {
            handle_connec(&socket_clone, manager_clone).await.unwrap();
        }
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
