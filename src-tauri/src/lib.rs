// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod compare;
pub mod ffuf;
pub mod juicy;
pub mod probing;
mod scan;
pub mod screenshot;
pub mod subdomains;
pub mod utils;
use scan::run_full_scan_command;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn ping() -> String {
    "Online".into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![ping, run_full_scan_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
