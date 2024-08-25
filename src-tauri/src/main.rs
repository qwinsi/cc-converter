// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use zhconv::{zhconv, Variant};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn tc2sc(input: &str) -> String {
    return zhconv(input, Variant::ZhHans)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tc2sc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
