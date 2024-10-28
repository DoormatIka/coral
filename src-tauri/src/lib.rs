use tauri::async_runtime::Mutex;

use tauri::{Manager, State};
use reqwest::{Client, Response, Error};
use std::collections::HashMap;

#[derive(Default)]
struct AppState {
    counter: u32,
    link: String,
    http_client: Client,
}

#[tauri::command]
fn change_link(link: &str, state: State<'_, Mutex<AppState>>) {
    let mut state = state.blocking_lock();
    state.link = link.to_string();
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str, state: State<'_, Mutex<AppState>>) -> String {
    let mut state = state.blocking_lock();
    state.counter += 1;
    format!("Hello, {}! You've been greeted from Rust! Called {} times.", name, state.counter)
}

#[tauri::command]
async fn create_ai_message(name: &str, state: State<'_, Mutex<AppState>>) -> Result<Response, Error> {
    let state = state.lock().await;
    // only one type for the value, which i dont want.
    let map = HashMap::<String, String>::new();
    let res = state.http_client.post(state.link.clone())
        .json(&map)
        .send()
        .await?;

    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, change_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
