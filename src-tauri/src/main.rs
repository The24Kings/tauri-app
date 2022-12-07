#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
    let mut counter = counter.0.lock().unwrap();

    *counter += num;
    format!("{}", counter)
}

fn main() {
    tauri::Builder::default()
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![greet, add_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
