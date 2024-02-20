// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use crate::db::Database;
use db::Password;
use rusqlite::ErrorCode;
use rusqlite::{params, Connection, Result, Row};
use std::fs;
use std::path::Path;

#[derive(Debug, serde::Serialize)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn copy_to_clipboard(text: &str) -> () {
    println!("Test");
}

#[tauri::command]
fn add_password(id: &str, title: &str, url: &str, username: &str, email: &str, password: &str , database: &str) -> Result<(), String> {
    // Connect to the database (consider using a more efficient way in a real app)
    let conn = Connection::open(database.to_string()).map_err(|e| e.to_string())?;

    // Insert a new user into the `user` table
    conn.execute(
        "INSERT INTO passwords (id, title, url, username, email, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, title, url, username, email, password],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn read_passwords(database: &str) -> Result<Vec<Password>, String> {
    let conn = Connection::open(database.to_string()).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, title, url, username, email, password FROM passwords")
        .map_err(|e| e.to_string())?;
    let password_iter = stmt
        .query_map([], |row| {
            Ok(Password {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                username: row.get(3)?,
                email: row.get(4)?,
                password: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut passwords = Vec::new();
    for password in password_iter {
        passwords.push(password.map_err(|e| e.to_string())?);
    }

    Ok(passwords)
}

#[tauri::command]
fn authorize(password: &str, database: &str) -> String {
    let db = match Database::new(password.to_string(), database.to_string()) {
        Ok(db) => db,
        Err(e) => {
            if e.sqlite_error_code().unwrap() == ErrorCode::NotADatabase {
                println!("passphrase is not valid!");
                std::process::exit(1);
            } else {
                println!("{}", e.to_string());
                std::process::exit(1);
            }
        }
    };
    let passwords = db.load();
    println!("{:?}", passwords);
    println!("Test");
    return String::from("testtest");
}

#[tauri::command]
fn list_databases(dirpath: &str) -> Result<Vec<String>, String> {
    let path = Path::new(dirpath);
    let mut db_files = Vec::new();

    // Attempt to read the directory and handle potential errors
    let entries =
        fs::read_dir(path).map_err(|e| format!("Failed to read directory '{}': {}", dirpath, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to process an entry: {}", e))?;
        let path = entry.path();
        // Check if it's a file and has a '.db' extension
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("db") {
            if let Some(str_path) = path.to_str() {
                db_files.push(str_path.to_owned());
            }
        }
    }
    Ok(db_files)
}

#[tauri::command]
fn select_database(dbPath: &str) -> Result<String, String> {
    println!("{}", dbPath);
    Ok(dbPath.to_string())
}

fn main() {
    // Connect to or create a new database
    let conn = Connection::open("my_database.db").unwrap();

    // Execute a simple SQL command to create a new table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER
         )",
        [],
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            copy_to_clipboard,
            add_password,
            list_databases,
            read_passwords,
            select_database,
            authorize
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
