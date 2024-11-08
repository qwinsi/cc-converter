// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};
use zhconv::{zhconv, Variant};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn tc2sc(input: &str) -> String {
    return zhconv(input, Variant::ZhHans);
}

#[tauri::command]
fn sc2tc(input: &str) -> String {
    return zhconv(input, Variant::ZhHant);
}

#[tauri::command]
fn tc2sc_phrase(input: &str) -> String {
    return zhconv(input, Variant::ZhCN);
}

#[tauri::command]
fn sc2tc_phrase(input: &str) -> String {
    return zhconv(input, Variant::ZhTW);
}

#[tauri::command]
fn quit(code: i32) {
    std::process::exit(code);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            tc2sc,
            sc2tc,
            tc2sc_phrase,
            sc2tc_phrase,
            quit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// https://v2.tauri.app/plugin/single-instance/
fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
