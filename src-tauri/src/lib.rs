mod api_server;
mod config;
mod front_data_type;
mod msg_type;
mod nc_signal;
mod setup;
mod tcp_client;
mod utils;

use reqwest::header::{self, HeaderMap};
use serde_json::Value;
use setup::set_up;
use std::sync::Arc;
use tauri::{command, Manager};
use tcp_client::TcpClientManager;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // 创建TcpClientManager
    let manager = Arc::new(Mutex::new(TcpClientManager::new()));

    tauri::Builder::default()
        .manage(manager)
        .setup(|_app| {
            let app_handle = _app.handle();
            let loading_window = _app.get_window("loading").unwrap();
            let window = _app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                let app_handle = app_handle.clone();
                set_up(app_handle).await;
                // 在新任务上执行初始化代码，这样应用程序就不会冻结
                std::thread::sleep(std::time::Duration::from_secs(5)); // 模拟初始化
                loading_window.close().unwrap();
                window.show().unwrap();
            });
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
    let config = config::get_config().await.unwrap();
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
