use uuid::Uuid;

use tauri::async_runtime::Mutex;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

use native_db::*;
use native_model::{native_model, Model};
use once_cell::sync::Lazy;

struct AppState {
    link: String,
    http_client: Client,
    char_db: Database<'static>,
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Character>().unwrap();
    models.define::<CharacterConversation>().unwrap();
    models
});

// everything returned from commands must implement serde::Serialize

///////////////////// CONVERSATION ///////////////////////

#[tauri::command]
fn grab_conversation(state: State<'_, Mutex<AppState>>, id: String) -> Result<CharacterConversation, String> {
    let state = state.blocking_lock();
    let transaction = state.char_db.r_transaction().unwrap();
    let conversation: CharacterConversation = transaction.get().primary(id).map_err(|err| err.to_string())?.ok_or_else(|| String::from("Chat does not exist!"))?;

    Ok(conversation)
}

#[tauri::command]
fn grab_conversation_list(state: State<'_, Mutex<AppState>>, char_id: String) -> Result<Vec<CharacterConversation>, String> {
    let state = state.blocking_lock();
    let transaction = state.char_db.r_transaction().unwrap();

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
async fn add_conversation(state: State<'_, Mutex<AppState>>, char_id: String) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state.char_db.rw_transaction().unwrap();

    let mut log: Vec<Message> = Vec::new();

    let char: Character = transaction
        .get()
        .primary(char_id.clone())
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from(""))?;

    log.push(Message { person: Person::System, content: char.system_message });
    log.push(Message { person: Person::User, content: String::new() });
    log.push(Message { person: Person::Assistant, content: char.first_message });

    let url = format!("http://{}/create", state.link);
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
        from_char_id: char_id,
        memory_id: new_memory_id,
        id: Uuid::new_v4().to_string(),
    };
    transaction
        .insert(conversation)
        .map_err(|err| err.to_string())?;
    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

/////////////// CHARACTER //////////////////////

#[tauri::command]
fn add_character(
    state: State<'_, Mutex<AppState>>,
    first_message: String,
    name: String,
    system_message: String,
    description: String,
) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state
        .char_db
        .rw_transaction()
        .map_err(|err| err.to_string())?;
    transaction
        .insert(Character {
            system_message,
            first_message,
            name,
            description,
            id: Uuid::new_v4().to_string(),
            conversations: Vec::new(),
        })
        .map_err(|err| err.to_string())?;

    transaction.commit().map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn grab_character_list(state: State<'_, Mutex<AppState>>) -> Result<Vec<Character>, String> {
    let state = state.blocking_lock();
    let transaction = state.char_db.r_transaction().unwrap();
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
    let transaction = state.char_db.r_transaction().unwrap();
    let char: Character = transaction
        .get()
        .primary(id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| String::from(""))?;

    Ok(char)
}

#[tauri::command]
fn delete_character(state: State<'_, Mutex<AppState>>, id: String) -> Result<(), String> {
    let state = state.blocking_lock();
    let transaction = state
        .char_db
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
fn change_link(link: &str, state: State<'_, Mutex<AppState>>) {
    let mut state = state.blocking_lock();
    state.link = link.to_string();
}

#[tauri::command]
async fn create_ai_message(
    conversationjson: &str,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let state = state.lock().await;

    let url = format!("http://{}/complete", state.link);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let character_db = Builder::new().create_in_memory(&MODELS).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let appstate = AppState {
                link: String::new(),
                http_client: Client::new(),
                char_db: character_db,
            };
            app.manage(Mutex::new(appstate));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            change_link,
            create_ai_message,

            grab_character,
            grab_character_list,

            grab_conversation,
            grab_conversation_list,

            add_character,
            delete_character,

            add_conversation,

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
