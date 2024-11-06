use transaction::RwTransaction;
use uuid::Uuid;

use tauri::async_runtime::Mutex;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

use native_db::*;
use native_model::{native_model, Model};
use once_cell::sync::Lazy;

use std::fs::{self, OpenOptions};
use std::path::Path;

struct AppState {
    http_client: Client,
    db: Database<'static>,
}

// will need all these types when i make client-side databases.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum Person {
    Assistant,
    System,
    User,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Message {
    person: Person,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct Character {
    #[primary_key]
    id: String,
    name: String,
    description: String,
    system_message: String,
    first_message: String,
    conversations: Vec<String>, // use IDs instead.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 2, version = 1)]
#[native_db]
struct CharacterConversation {
    #[primary_key]
    id: String,
    #[secondary_key]
    from_char_id: String,
    memory_id: String,
    log: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[native_model(id = 3, version = 1)]
#[native_db]
struct Settings {
    #[primary_key]
    id: String,
    link: String,
    temp: f32,
    top_p: f32,
    top_k: f32,
    min_p: f32,
    typical_p: f32,
    repeat_penalty: f32,
    tfs_z: f32,
    mirostat_mode: f32,
    mirostat_tau: f32,
    mirostat_eta: f32,
}

static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Character>().unwrap();
    models.define::<CharacterConversation>().unwrap();
    models.define::<Settings>().unwrap();
    models
});

// everything returned from commands must implement serde::Serialize

///////////////////// CONVERSATION ///////////////////////

#[tauri::command]
fn grab_conversation(
    state: State<'_, Mutex<AppState>>,
    chat_id: String,
) -> Result<CharacterConversation, String> {
    let state = state.blocking_lock();
    let transaction = state.db.r_transaction().unwrap();
    let conversation: CharacterConversation = transaction
        .get()
        .primary(chat_id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from("Chat does not exist!"))?;

    Ok(conversation)
}

#[tauri::command]
async fn update_conversation_log(
    state: State<'_, Mutex<AppState>>,
    chat_id: String,
    log: Vec<Message>,
) -> Result<(), String> {
    let state = state.lock().await;
    let transaction = state.db.rw_transaction().unwrap();
    let conversation: CharacterConversation = transaction
        .get()
        .primary(chat_id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from("Chat does not exist!"))?;
    transaction
        .update(
            conversation.clone(),
            CharacterConversation {
                log,
                id: conversation.id,
                from_char_id: conversation.from_char_id,
                memory_id: conversation.memory_id,
            },
        )
        .map_err(|err| err.to_string())?;
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn grab_conversation_list(
    state: State<'_, Mutex<AppState>>,
    char_id: String,
) -> Result<Vec<CharacterConversation>, String> {
    let state = state.blocking_lock();
    let transaction = state.db.r_transaction().unwrap();

    let char_list: Vec<CharacterConversation> = transaction
        .scan()
        .secondary::<CharacterConversation>(CharacterConversationKey::from_char_id)
        .map_err(|err| err.to_string())?
        .start_with(char_id)
        .map_err(|err| err.to_string())?
        .map(|v| v.unwrap())
        .collect();

    Ok(char_list)
}

#[tauri::command]
async fn add_conversation(
    state: State<'_, Mutex<AppState>>,
    char_id: String,
) -> Result<String, String> {
    let state = state.lock().await;
    let transaction = state.db.rw_transaction().unwrap();

    let mut log: Vec<Message> = Vec::new();
    let id = Uuid::new_v4().to_string();

    let char: Character = transaction
        .get()
        .primary(char_id.clone())
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from(""))?;

    log.push(Message {
        person: Person::System,
        content: char.system_message,
    });
    log.push(Message {
        person: Person::User,
        content: String::new(),
    });
    log.push(Message {
        person: Person::Assistant,
        content: char.first_message,
    });

    let settings = get_maybe_settings(&transaction);

    let url = format!("http://{}/create", settings.link);
    let new_memory_id = state
        .http_client
        .get(url)
        .send()
        .await
        .map_err(|err| err.to_string())?
        .text()
        .await
        .map_err(|err| err.to_string())?;

    let conversation = CharacterConversation {
        log,
        id: id.clone(),
        from_char_id: char_id,
        memory_id: new_memory_id,
    };
    transaction
        .insert(conversation)
        .map_err(|err| err.to_string())?;
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(id)
}

/////////////// CHARACTER //////////////////////

#[tauri::command]
fn add_character(
    state: State<'_, Mutex<AppState>>,
    name: String,
    description: String,
    first_message: String,
    system_message: String,
) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state
        .db
        .rw_transaction()
        .map_err(|err| err.to_string())?;
    transaction
        .insert(Character {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            system_message,
            first_message,
            conversations: Vec::new(),
        })
        .map_err(|err| err.to_string())?;

    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn grab_character_list(state: State<'_, Mutex<AppState>>) -> Result<Vec<Character>, String> {
    let state = state.blocking_lock();
    let transaction = state.db.r_transaction().unwrap();
    let chars = transaction
        .scan()
        .primary::<Character>()
        .map_err(|err| err.to_string())?
        .all()
        .map_err(|err| err.to_string())?
        .map(|v| v.unwrap())
        .collect();

    Ok(chars)
}

#[tauri::command]
fn grab_character(state: State<'_, Mutex<AppState>>, id: String) -> Result<Character, String> {
    let state = state.blocking_lock();
    let transaction = state.db.r_transaction().unwrap();
    let char: Character = transaction
        .get()
        .primary(id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from(""))?;

    Ok(char)
}

#[tauri::command]
fn update_character(
    state: State<'_, Mutex<AppState>>,
    char_id: String,
    name: Option<String>,
    description: Option<String>,
    system_message: Option<String>,
    first_message: Option<String>,
    conversations: Option<Vec<String>>,
) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state.db.rw_transaction().unwrap();
    let char: Character = transaction
        .get()
        .primary(char_id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from(""))?;
    transaction
        .update(
            char.clone(),
            Character {
                id: char.id,
                name: name.unwrap_or(char.name),
                description: description.unwrap_or(char.description),
                system_message: system_message.unwrap_or(char.system_message),
                first_message: first_message.unwrap_or(char.first_message),
                conversations: conversations.unwrap_or(char.conversations),
            },
        )
        .map_err(|err| err.to_string())?;
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn delete_character(state: State<'_, Mutex<AppState>>, id: String) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state
        .db
        .rw_transaction()
        .map_err(|err| err.to_string())?;
    let character: Character = transaction
        .get()
        .primary(id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from(""))?;
    transaction
        .remove(character)
        .map_err(|err| err.to_string())?;
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

/////////////////// ESSENTIALS //////////////////

#[tauri::command]
fn change_settings(
    state: State<'_, Mutex<AppState>>,
    link: String,
    temp: f32,
    top_p: f32,
    top_k: f32,
    min_p: f32,
    typical_p: f32,
    repeat_penalty: f32,
    tfs_z: f32,
    mirostat_mode: f32,
    mirostat_tau: f32,
    mirostat_eta: f32,
) -> Result<(), String> {
    let settings = Settings {
        id: String::from("123"),
        link,
        temp,
        top_p,
        top_k,
        min_p,
        typical_p,
        repeat_penalty,
        tfs_z,
        mirostat_mode,
        mirostat_tau,
        mirostat_eta,
    };
    let state = state.blocking_lock();
    let transaction = state
        .db
        .rw_transaction()
        .map_err(|err| err.to_string())?;
    transaction
        .upsert(settings)
        .map_err(|err| err.to_string())?;
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

fn get_maybe_settings(transaction: &RwTransaction<'_>) -> Settings {
    let settings: Option<Settings> = transaction
        .get()
        .primary(String::from("123"))
        .unwrap_or_else(|err| {
            match err {
                native_db::db_type::Error::ModelError(_) => {
                    let mut settings = Settings::default();
                    settings.id = String::from("123");
                    transaction.insert(settings.clone()).unwrap();
                    println!("{:?}", err);

                    Some(settings)
                }
                _ => {
                    println!("{:?}", err);
                    None
                },
            }
        });
    let settings = match settings {
        Some(settings) => settings,
        None => {
            let mut settings = Settings::default();
            settings.id = String::from("123");
            transaction.insert(settings.clone()).unwrap();

            settings
        }
    };

    settings
}

#[tauri::command]
fn grab_settings(state: State<'_, Mutex<AppState>>) -> Result<Settings, String> {
    let state = state.blocking_lock();
    let transaction = state
        .db
        .rw_transaction()
        .map_err(|err| err.to_string())?;
    let settings = get_maybe_settings(&transaction);
    transaction.commit().map_err(|err| err.to_string()).unwrap();

    Ok(settings)
}

#[tauri::command]
async fn create_ai_message(
    state: State<'_, Mutex<AppState>>,
    conversationjson: &str,
) -> Result<String, String> {
    let state = state.lock().await;
    let transaction = state
        .db
        .rw_transaction()
        .map_err(|err| err.to_string())?;

    let settings = get_maybe_settings(&transaction);
    transaction.commit().map_err(|err| err.to_string())?;

    let url = format!("http://{}/complete", settings.link);
    let res = state
        .http_client
        .post(url)
        .body(conversationjson.to_string())
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|err| err.to_string())?
        .text()
        .await
        .map_err(|err| err.to_string())?;

    Ok(res)
}

#[tauri::command]
fn clear_all(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state
        .db
        .rw_transaction()
        .map_err(|err| err.to_string())?;

    let all: Vec<Character> = transaction
        .scan()
        .primary()
        .map_err(|err| err.to_string())?
        .all()
        .map_err(|err| err.to_string())?
        .map(|v| v.unwrap())
        .collect();
    for elem in all {
        transaction.remove(elem).map_err(|err| err.to_string())?;
    }
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

fn read_or_create_file(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directory.");
        }
    }
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let dir = app.path().app_local_data_dir().expect("couldn't resolve app data dir").join("data");

            read_or_create_file(dir.to_str().unwrap()).unwrap();
            let db = Builder::new().create(&MODELS, dir).unwrap();
            let appstate = AppState { db, http_client: Client::new() };

            app.manage(Mutex::new(appstate));

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            change_settings,
            grab_settings,
            create_ai_message,
            grab_character,
            grab_character_list,
            grab_conversation,
            grab_conversation_list,
            add_character,
            delete_character,
            update_character,
            add_conversation,
            update_conversation_log,
            clear_all,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
