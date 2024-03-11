// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use crate::db::Database;
use db::VaultEntry;
use rusqlite::Result;
use std::sync::{Arc, Mutex};
use tauri::State;

// Wrap the database in Arc<Mutex<>> for thread-safe sharing
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Database>>,
}

#[tauri::command]
fn add_vault_entry(
    state: State<AppState>,
    title: String,
    url: String,
    username: String,
    email: String,
    password: String,
    master_password_id: String,
    vault: String,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.add_vault_entry(
        vault,
        title,
        url,
        username,
        email,
        password,
        master_password_id,
    )
}

#[tauri::command]
fn read_entries(state: State<AppState>, vault: &str) -> Result<Vec<VaultEntry>, String> {
    let db = state.db.lock().unwrap();
    db.read_entries(vault)
}

#[tauri::command]
fn list_vaults(state: State<AppState>) -> Result<Vec<String>, String> {
    let db = state.db.lock().unwrap();
    db.list_vaults()
}

#[tauri::command]
fn add_vault(state: State<AppState>, name: &str, password: &str) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.add_vault(name, password)
}

#[tauri::command]
fn select_vault(
    state: State<AppState>,
    name: &str,
    master_password: &str,
) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    db.select_vault(name, master_password)
}

fn main() {
    // Connect to or create a new database
    let db = Database::init().expect("Failed to initialize database");
    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            add_vault_entry,
            add_vault,
            list_vaults,
            read_entries,
            select_vault,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
