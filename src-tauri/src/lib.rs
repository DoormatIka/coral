use tauri::async_runtime::Mutex;

use tauri::{Manager, State};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use native_db::*;
use native_model::{native_model, Model};
use once_cell::sync::Lazy;
use transaction::query::PrimaryScan;

struct AppState {
    counter: u32,
    link: String,
    http_client: Client,
    db: Database<'static>,
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
    memories: Vec<Message>,
    log: Vec<Message>,
    regen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct Character {
    #[primary_key]
    id: u32,
    name: String,
    system_message: String,
    conversation: CharacterConversation,
}

#[derive(Serialize, Deserialize, Debug)]
struct CharacterConversation {
    memory_id: String,
    memories: Vec<Message>,
    log: Vec<Message>,
}

static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Character>().unwrap();
    models
});

// everything returned from commands must implement serde::Serialize

#[tauri::command]
fn grab_character_list(state: State<'_, Mutex<AppState>>) -> Result<Vec<Character>, String> {
    let state = state.blocking_lock();
    let transaction = state.db.r_transaction().unwrap();
    let list = transaction.scan().primary::<Character>().unwrap();
    let chars: Vec<Character> = list
        .all()
        .map_err(|err| { err.to_string() })?
        .map(|v| v.unwrap())
        .collect();

    Ok(chars)
}

#[tauri::command]
fn grab_character(state: State<'_, Mutex<AppState>>, id: u32) -> Character {
    let state = state.blocking_lock();
    let transaction = state.db.r_transaction().unwrap();
    let char: Character = transaction.get().primary(id).unwrap().unwrap();
    
    char
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
async fn create_ai_message(conversationjson: &str, state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().await;

    let url = format!("http://{}/complete", state.link);

    let res = state.http_client
        .post(url)
        .body(conversationjson.to_string())
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|err| { err.to_string() })?
        .text()
        .await
        .map_err(|err| { err.to_string() })?;

    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Builder::new().create_in_memory(&MODELS).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let appstate = AppState {
                counter: 0,
                link: String::new(),
                http_client: Client::new(),
                db,
            };
            app.manage(Mutex::new(appstate));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, change_link, create_ai_message, grab_character_list, grab_character])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
