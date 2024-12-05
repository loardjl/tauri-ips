use crate::msg_type::{GetToken, MsgType, OptimizeInfo, ProtocolHeader, RealTimeData};
use crate::nc_signal::{set_nc_signal_val, NcSignal, NcSignalManager, NcSignalVal};
use crate::tcp_client::TcpClientManager;

use axum::routing::get;
use bincode::serialize;
use lazy_static::lazy_static;
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
    static ref NC_SIGNAL_MANAGER: Mutex<NcSignalManager> = Mutex::new(NcSignalManager::new());
}

async fn send_tcp(manager: TcpClientManager1, data: Vec<u8>, topic: &str) {
    let manager_clone = manager.clone();
    println!("Received message: {:?}", data);
    let manager = manager_clone.lock().await;
    if let Some(client) = manager.get_client(topic) {
        let client = client.clone();
        drop(manager); // 释放 manager 的锁
        println!("Sending data to TCP client: {:?}", data);
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
            println!("Client '{}' not found", client_name);
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
                                    println!("Received response (JSON): {:?}", parsed_response);
                                    socket.emit(msg_str, &parsed_response).ok();
                                }
                                Err(e) => eprintln!("Failed to parse JSON response: {}", e),
                            }
                        } else {
                            let decoded = decode_data(data, &msg, &topic).await;
                            println!("Received response: {:?}", decoded);
                            socket.emit(msg_str, &decoded).ok();
                        }
                    }
                    Err(e) => eprintln!("Failed to deserialize SendData: {}", e),
                }
            }
            None => {
                println!("TCP client '{}' receiver closed.", client_name);
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
            println!("Client '{}' not found", client_name);
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
        println!("Received IpsRegister message: {:?}", data);
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
        println!("Received getToken message: {:?}", data);
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
    // 启动 TCP 消息接收与连接状态维护
    tokio::spawn(handle_tcp_messages("dc", manager.clone(), socket.clone()));
    tokio::spawn(maintain_connection("dc", manager.clone()));
    tokio::spawn(handle_tcp_messages("ips", manager.clone(), socket.clone()));
    tokio::spawn(maintain_connection("ips", manager.clone()));

    Ok(())
}

async fn decode_data(data: Vec<u8>, msg: &MsgType, topic: &str) -> Result<Value, String> {
    println!("Decoding data: {:?}", data);
    println!("Decoding msg: {:?}", msg);
    match msg {
        MsgType::RealTimeData => {
            let result = RealTimeData::parse_real_time_data(&data);
            match result {
                Ok(value) => {
                    println!("Decoded RealTimeData: {:?}", value);
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
                    println!("Decoded OptimizeInfo: {:?}", value);
                    let value = json!(value);
                    Ok(value)
                }
                Err(e) => Err(format!("Failed to parse OptimizeInfo: {}", e)),
            }
        }
        MsgType::NcSignalVal => {
            let signal_value = NcSignal::parse_nc_signal(data.as_slice()).unwrap();
            if topic == "dc" {
                let nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
                let signal_data = nc_signal_manager.realtime_data.get("dc").cloned();
                drop(nc_signal_manager);
                match signal_data {
                    Some(mut signal_data) => {
                        println!("update signal_data: {:?}", signal_data);
                        set_nc_signal_val(&mut signal_data, signal_value);
                        let mut nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
                        nc_signal_manager
                            .realtime_data
                            .insert("dc".to_string(), signal_data);
                    }
                    None => {
                        let nc_signal_val =
                            NcSignalVal::new(signal_value.dev_id, signal_value.collector_id);
                        let mut realtime_data = HashMap::new();
                        realtime_data.insert(signal_value.collector_id, nc_signal_val);
                        set_nc_signal_val(&mut realtime_data, signal_value);
                        let mut nc_signal_manager = NC_SIGNAL_MANAGER.lock().await;
                        nc_signal_manager
                            .realtime_data
                            .insert("dc".to_string(), realtime_data);
                        println!("insert signal_data: {:?}", nc_signal_manager.realtime_data);
                    }
                }
            }
            Ok(json!(""))
        }
        MsgType::GetToken => {
            let result = GetToken::parse_get_token(&data);
            match result {
                Ok(value) => {
                    println!("Decoded GetToken: {:?}", value);
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
