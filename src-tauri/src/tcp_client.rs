use crate::msg_type::{
    get_msg, get_process_mode, parse_protocol_header, MsgType, ProcessingMode, ProtocolHeader,
};
use crate::nc_signal::NcSignalManager;
use bincode::Options;
use futures_util::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{self, Duration};

#[derive(Debug, Deserialize, Clone)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
}
pub struct TcpClient {
    address: String,
    sender: Arc<Mutex<Sender<Vec<u8>>>>,
    receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
    writer: Option<Arc<Mutex<OwnedWriteHalf>>>, // 存储可写流
    status: Arc<Mutex<i32>>,                    // 连接状态，1 为连接成功，0 为断开
    heartbeat: ProtocolHeader,
    topic: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SliceData {
    slice_count: u16, // 分片总数
    slice_id: u16,    // 分片
    slice_size: u16,  // 分片大小
    val: Vec<u8>,     // 数据
}

impl TcpClient {
    /// 创建新的 TCP 客户端
    pub async fn new(
        config: TcpConfig,
        heartbeat: ProtocolHeader,
        topic: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let address = format!("{}:{}", config.host, config.port);
        let (tx, rx) = mpsc::channel(1000);
        let mut client = TcpClient {
            address,
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
            writer: None,
            status: Arc::new(Mutex::new(0)), // 默认断开
            heartbeat,
            topic,
        };

        // 尝试启动监听
        client.start_listening().await.map_err(|e| {
            eprintln!("Failed to start listening: {}", e);
            e
        })?;

        Ok(client)
    }

    /// 发送数据到服务器
    pub async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(writer) = &self.writer {
            let mut writer = writer.lock().await;
            let hex_output: String = data
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<_>>()
                .join(" ");
            println!("Sending data: {:?}", hex_output);
            writer.write_all(data).await?; // 写入数据
            Ok(())
        } else {
            Err("Writer not initialized".into())
        }
    }

    /// 开始监听消息和心跳任务
    pub async fn start_listening(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("topic: {:?}", self.topic);
        loop {
            match self.connect_and_listen().await {
                Ok(_) => break, // 成功连接
                Err(e) => {
                    eprintln!(
                        "{} Failed to connect: {}. Retrying in 5 seconds...",
                        self.topic, e
                    );
                    time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
        Ok(())
    }

    /// 连接并监听消息
    pub async fn connect_and_listen(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let address = self.address.clone();
        let sender = self.sender.clone();
        let status = self.status.clone();

        // 尝试连接服务器
        let stream = TcpStream::connect(&address).await?;
        let (mut reader, writer) = stream.into_split();
        self.writer = Some(Arc::new(Mutex::new(writer))); // 现在可以正确初始化 writer

        // 连接成功，设置状态为 1
        let mut status_lock = self.status.lock().await;
        *status_lock = 1;
        drop(status_lock);
        let topic = self.topic.clone();
        println!("connect_and_listen topic: {:?}", topic);

        // 消息接收任务
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let buf_size = 65550;
            let mut buffer = vec![0; buf_size];
            let mut leftover = vec![]; // 用于存储上次未处理完的数据

            // 创建 FragmentationHandler 实例
            let mut fragmentation_handler = None;

            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Server closed the connection.");
                        let mut status = status.lock().await;
                        *status = 0;
                        break; // 连接关闭
                    }
                    Ok(n) => {
                        // 合并上次残留数据和本次读取的数据
                        leftover.extend_from_slice(&buffer[..n]);
                        println!("leftover----------: {:?}", leftover);

                        // 循环解析有效数据包
                        while leftover.len() >= 15 {
                            if let Some(pos) =
                                leftover.windows(2).position(|window| window == [235, 144])
                            {
                                if leftover.len() < pos + 15 {
                                    break; // 数据不足，等待更多数据
                                }

                                let header_bytes = &leftover[pos..pos + 15];
                                let header = match parse_protocol_header(header_bytes) {
                                    Ok(hdr) => hdr,
                                    Err(e) => {
                                        eprintln!("Failed to deserialize header: {}", e);
                                        return;
                                    }
                                };

                                let data_len = header.len as usize;
                                let total_len = pos + 15 + data_len;

                                if leftover.len() < total_len {
                                    break; // 数据不足，等待更多数据
                                }
                                let hex_output: String = leftover
                                    .iter()
                                    .map(|byte| format!("{:02X}", byte))
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                println!("Received data: {:?}", hex_output);

                                // 解析 order1 和 order2
                                let order1 = u16::from_be_bytes([leftover[3], leftover[4]]);
                                let order2 = u16::from_be_bytes([leftover[5], leftover[6]]);
                                println!(
                                    "order1: {:04X} order2: {:04X} topic: {}",
                                    order1, order2, &topic
                                );
                                // 判断数据处理模式
                                let processing_mode = get_process_mode(order1, order2);
                                let msg = get_msg(order1, order2);

                                println!("Processing mode: {:?}", processing_mode);
                                process_data_segment(
                                    &mut leftover,
                                    pos,
                                    data_len,
                                    sender_clone.clone(),
                                    processing_mode,
                                    &mut fragmentation_handler,
                                    msg,
                                )
                                .await;
                            } else {
                                leftover.clear();
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading: {}", e);
                        let mut status = status.lock().await;
                        *status = 0;
                        break;
                    }
                }
            }
        });
        let hearbeat = self.heartbeat.clone();
        let topic = self.topic.clone();
        // 心跳任务
        if let Some(writer) = &self.writer {
            let writer_clone = Arc::clone(writer);
            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_secs(5));
                loop {
                    interval.tick().await;
                    let mut writer = writer_clone.lock().await;

                    let hearbeat_bytes = bytemuck::bytes_of(&hearbeat);
                    eprintln!("hearbeat_bytes: {:?}", &topic);
                    if let Err(e) = writer.write_all(&hearbeat_bytes).await {
                        eprintln!("Error sending heartbeat: {}", e);
                        break; // 连接断开，停止心跳任务
                    } else {
                        // println!("Sent heartbeat");
                    }
                }
            });
        }

        Ok(())
    }
    // 重连
    pub async fn watch_connection(&mut self) {
        {
            // 每次循环加锁
            let status = self.status.lock().await;
            let disconnected = *status == 0;
            // 锁在此处释放
            drop(status);
            if disconnected {
                println!("Disconnected. Reconnecting...");
                // 重连逻辑
                match self.connect_and_listen().await {
                    Ok(_) => println!("Reconnected successfully."),
                    Err(e) => eprintln!("Failed to reconnect: {}", e),
                }
            } else {
                // println!("Connection is healthy.");
            }
        }
    }

    /// 获取接收器
    pub fn get_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<u8>>>> {
        Arc::clone(&self.receiver)
    }
    pub fn get_topic(&self) -> String {
        self.topic.clone()
    }
}

async fn process_data_segment(
    leftover: &mut Vec<u8>,
    pos: usize,
    data_len: usize,
    sender_clone: Arc<Mutex<Sender<Vec<u8>>>>,
    processing_mode: ProcessingMode,
    fragmentation_handler: &mut Option<Arc<Mutex<FragmentationHandler>>>,
    msg: MsgType,
) {
    let total_len = pos + 15 + data_len;

    if leftover.len() < total_len {
        return; // 数据不足，等待更多数据
    }

    let data_bytes = &leftover[pos + 15..total_len];
    println!("data_bytes: {:?}", data_bytes);

    // 根据处理模式进行不同的处理
    match processing_mode {
        ProcessingMode::FragmentedJson | ProcessingMode::Fragmented => {
            let mut slice_data: SliceData = bincode::options()
                .with_big_endian()
                .allow_trailing_bytes()
                .deserialize(data_bytes)
                .unwrap();
            let slice_val = &data_bytes[6..];
            slice_data.val = slice_val.to_vec();
            println!("slice_data: {:?}", slice_data);

            // 初始化分片处理器
            if fragmentation_handler.is_none() {
                *fragmentation_handler = Some(Arc::new(Mutex::new(FragmentationHandler::new(
                    slice_data.slice_count,
                ))));
            }

            // 提取分片数据并处理
            let mut handler = fragmentation_handler.clone().unwrap();
            if handler.lock().await.total_count == 0 {
                *fragmentation_handler = Some(Arc::new(Mutex::new(FragmentationHandler::new(
                    slice_data.slice_count,
                ))));
                handler = fragmentation_handler.clone().unwrap();
            }
            handle_fragmented_data(
                handler,
                slice_data.val,
                slice_data.slice_id,
                sender_clone.clone(),
                processing_mode == ProcessingMode::FragmentedJson,
                msg,
            )
            .await;
        }
        ProcessingMode::DirectJson | ProcessingMode::Direct => {
            let send_data = SendData {
                data: data_bytes.to_vec(),
                is_json: processing_mode == ProcessingMode::DirectJson,
                msg,
            };
            let serialized_data = serde_json::to_vec(&send_data).expect("Failed to serialize data");
            if let Err(e) = sender_clone.lock().await.send(serialized_data).await {
                eprintln!("Failed to send data: {}", e);
            }
        }
    }

    // 移除已处理的数据
    *leftover = leftover.split_off(total_len);
}

#[derive(Debug, Serialize, Deserialize)]
struct SendData {
    data: Vec<u8>,
    is_json: bool,
    msg: MsgType,
}

#[derive(Debug)]
struct FragmentationHandler {
    fragments: HashMap<u16, Vec<u8>>, // 存储分片数据，按分片ID索引
    total_count: u16,                 // 总共分了多少片
    current_count: u16,               // 当前已经接收了多少片
}

impl FragmentationHandler {
    fn new(total_count: u16) -> Self {
        FragmentationHandler {
            fragments: HashMap::new(),
            total_count,
            current_count: 0,
        }
    }

    fn add_fragment(&mut self, slice_id: u16, data: Vec<u8>) {
        if !self.fragments.contains_key(&slice_id) {
            self.fragments.insert(slice_id, data);
            self.current_count += 1;
        }
    }

    // 合并数据，如果所有分片都到达，则返回合并后的数据
    fn try_to_merge(&mut self) -> Option<Vec<u8>> {
        println!(
            "current_count: {:?}, total_count: {:?}",
            self.current_count, self.total_count
        );
        if self.current_count == self.total_count {
            // 合并所有分片
            let mut merged_data = Vec::new();
            for i in 0..self.total_count + 1 {
                if let Some(fragment) = self.fragments.remove(&(i)) {
                    merged_data.extend(fragment);
                }
            }
            Some(merged_data)
        } else {
            None
        }
    }
}

async fn handle_fragmented_data(
    client: Arc<Mutex<FragmentationHandler>>,
    data: Vec<u8>,
    slice_id: u16,
    sender: Arc<Mutex<Sender<Vec<u8>>>>, // 用于发送数据
    is_json: bool,
    msg: MsgType,
) {
    let mut handler = client.lock().await;
    handler.add_fragment(slice_id, data);

    // 如果分片合并完成，则发送数据
    if let Some(merged_data) = handler.try_to_merge() {
        println!("All fragments received. Merged data: {:?}", merged_data);
        let send_data = SendData {
            data: merged_data,
            is_json,
            msg,
        };
        let serialized_data = serde_json::to_vec(&send_data).expect("Failed to serialize data");
        // 通过 sender 发送合并后的数据
        if let Err(e) = sender.lock().await.send(serialized_data).await {
            eprintln!("Failed to send merged data: {}", e);
        }
        handler.current_count = 0;
        handler.total_count = 0;
        handler.fragments.clear();
    } else {
        println!("Waiting for more fragments...");
    }
}

pub struct TcpClientManager {
    pub clients: HashMap<String, Arc<Mutex<TcpClient>>>,
}

impl TcpClientManager {
    pub fn new() -> Self {
        TcpClientManager {
            clients: HashMap::new(),
        }
    }

    pub async fn add_client(
        &mut self,
        topic: String,
        config: TcpConfig,
        heartbeat: ProtocolHeader,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = TcpClient::new(config, heartbeat, topic.clone()).await?;
        self.clients
            .insert(topic.clone(), Arc::new(Mutex::new(client)));
        Ok(())
    }

    pub fn get_client(&self, topic: &str) -> Option<Arc<Mutex<TcpClient>>> {
        self.clients.get(topic).cloned()
    }
}
