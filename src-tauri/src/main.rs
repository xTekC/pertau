#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn from_tauri(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Tauri!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![from_tauri])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
