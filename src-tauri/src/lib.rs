use serde_json::json;
use tauri::async_runtime::Mutex;

use tauri::{Manager, State};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct AppState {
    counter: u32,
    link: String,
    http_client: Client,
}

// will need all these types when i make client-side databases.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Person {
    Assistant,
    System,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    person: Person,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Conversation {
    memory_id: String,
    log: Vec<Message>,
    regen: bool,
}

// everything returned from commands must implement serde::Serialize



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
async fn create_ai_message(conversationjson: &str, state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().await;

    let mut link = String::new();
    link.push_str("http://");
    link.push_str(&state.link);
    link.push_str("/complete");

    let res = state.http_client.post(link)
        .body(conversationjson.to_string())
        .header("Content-Type", "application/json")
        .send()
        .await;
    let res = match res {
        Ok(res) => res.text().await,
        Err(err) => {
            println!("{}", err);
            return Err(err.to_string());
        }
    };
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return Err(err.to_string());
        }
    };

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
        .invoke_handler(tauri::generate_handler![greet, change_link, create_ai_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
