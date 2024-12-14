use log4rs;
use tauri::AppHandle;

pub fn main(handle: &AppHandle) {
    let handle_clone = handle.clone();
    let config_path = handle_clone
        .path_resolver()
        .resolve_resource("config/log.yml")
        .expect("failed to resolve resource");
    log4rs::init_file(config_path, Default::default()).unwrap();
}
