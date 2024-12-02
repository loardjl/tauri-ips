use crate::tcp_client::TcpClientManager;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};
use warp::Filter;

type TcpClientManager1 = Arc<Mutex<TcpClientManager>>;

///监听连接
async fn handle_ws_connection(ws: WebSocket, manager: TcpClientManager1) {
    //将ws分割为 两部分  rx读 tx 写
    let (mut ws_tx, mut ws_rx) = ws.split();
    //创建一个无界通道 用于不同异步任务中的收发消息
    let (tx, mut rx) = mpsc::unbounded_channel();
    //避免所有权转移导致不可用 所以clone一个manager
    let manager_clone = manager.clone();
    tokio::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(msg) => {
                    if let Ok(text) = msg.to_str() {
                        // 假设前端发送的数据是 JSON 格式，包含 topic 和 data 字段
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                            if let (Some(topic), Some(data)) = (json.get("topic"), json.get("data"))
                            {
                                if let (Some(topic), Some(data)) = (topic.as_str(), data.as_str()) {
                                    let manager = manager_clone.lock().await;
                                    if let Some(client) = manager.get_client(topic) {
                                        let client = client.lock().await;
                                        client
                                            .send_data(data)
                                            .expect("Failed to send data to TCP server");
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            ws_tx
                .send(Message::text(msg))
                .await
                .expect("Failed to send WebSocket message");
        }
    });

    let manager = manager.lock().await;
    for client in manager.clients.values() {
        let client = client.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let client = client.lock().await;
            let receiver = client.get_receiver();
            while let Ok(msg) = receiver.recv() {
                tx.send(msg).expect("Failed to send message to WebSocket");
            }
        });
    }
}

///
/// # 参数
///
/// - `ws`: 一个 `warp::ws::Ws` 类型的 WebSocket 实例。
///
/// # 返回值
///
/// 返回一个处理 WebSocket 连接的闭包。
pub async fn run_ws_server(port: u16, manager: TcpClientManager1) {
    // 这个 `ws_route` 定义了一个 WebSocket 路由。
    // 它使用 `warp::path("ws")` 来匹配路径为 "ws" 的请求，并且使用 `warp::ws()` 来处理 WebSocket 协议。
    // 在请求匹配后，它会调用 `map` 方法，将 WebSocket 协议升级为一个 WebSocket 连接。
    // 在连接升级过程中，它会克隆一个 `manager` 实例，并调用 `handle_ws_connection` 函数来处理 WebSocket 连接。
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let manager = manager.clone();
            ws.on_upgrade(move |socket| handle_ws_connection(socket, manager))
        });
    println!("WebSocket server is running on ws:// {:?}", port);
    warp::serve(ws_route).run(([127, 0, 0, 1], port)).await;
}
