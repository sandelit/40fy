// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, Result, Row};

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
fn create_user(name: String, age: i32) -> Result<(), String> {
    // Connect to the database (consider using a more efficient way in a real app)
    let conn = Connection::open("my_database.db")
        .map_err(|e| e.to_string())?;

    // Insert a new user into the `user` table
    conn.execute(
        "INSERT INTO user (name, age) VALUES (?1, ?2)",
        params![name, age],
    ).map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
fn read_users() -> Result<Vec<User>, String> {
    let conn = Connection::open("my_database.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare("SELECT id, name, age FROM user").map_err(|e| e.to_string())?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user.map_err(|e| e.to_string())?);
    }

    Ok(users)
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
        .invoke_handler(tauri::generate_handler![copy_to_clipboard, create_user, read_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
