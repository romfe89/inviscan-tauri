// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod compare;
pub mod ffuf;
pub mod juicy;
pub mod probing;
pub mod commands;
mod scan;
pub mod screenshot;
pub mod subdomains;
pub mod utils;
use scan::run_full_scan_command;
use commands::get_previous_scans;

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
        .invoke_handler(tauri::generate_handler![
            greet,
            ping,
            run_full_scan_command,
            get_previous_scans
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
