// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use crate::db::Database;
use db::VaultEntry;
use rusqlite::ErrorCode;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, serde::Serialize)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[tauri::command]
fn add_password(
    id: &str,
    title: &str,
    url: &str,
    username: &str,
    email: &str,
    password: &str,
    database: &str,
) -> Result<(), String> {
    // Connect to the database (consider using a more efficient way in a real app)
    let conn = Connection::open(database.to_string()).map_err(|e| e.to_string())?;

    // Insert a new user into the `user` table
    conn.execute(
        "INSERT INTO Password (id, title, url, username, email, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, title, url, username, email, password],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn read_passwords(database: &str) -> Result<Vec<VaultEntry>, String> {
    let conn = Connection::open(database.to_string()).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, title, url, username, email, password FROM passwords")
        .map_err(|e| e.to_string())?;
    let password_iter = stmt
        .query_map([], |row| {
            Ok(VaultEntry {
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
    unimplemented!();
}

#[tauri::command]
fn list_vaults() -> Result<Vec<String>, String> {
    // Attempt to connect to the SQLite database
    let conn = Connection::open("./passwords.db")
        .map_err(|e| e.to_string())?;

    // Prepare the SQL query to select all user-created table names
    let mut statement = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'MasterPassword'")
        .map_err(|e| e.to_string())?;

    // Execute the query and collect table names into a Vec<String>
    let tables = statement.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<String>, _>>()
    .map_err(|e| e.to_string())?;

    for table in &tables {
        println!("Table: {}", table);
    }
    Ok(tables)
}

#[tauri::command]
fn add_vault(name: &str, password: &str) -> Result<(), String> {
    let conn = rusqlite::Connection::open("./passwords.db")
        .map_err(|e| e.to_string())?;
    
    // Insert into master table and get the ID from it
    conn.execute(
        "INSERT INTO MasterPassword (password) VALUES (?1)",
        &[&password],
    )
    .map_err(|e| e.to_string())?;

    //Create new user-generated table using the master_password_id as a foreign key
    let statement = format!("CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY,
        master_password INTEGER,
        title TEXT NOT NULL,
        url TEXT,
        username TEXT,
        email TEXT,
        password TEXT,
        FOREIGN KEY(master_password) REFERENCES MasterPassword(id)
    )", name);

    conn.execute(&statement, rusqlite::params![])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn select_vault(vault_name: &str) -> Result<Vec<VaultEntry>, String> {
    let conn = rusqlite::Connection::open("./passwords.db")
        .map_err(|e| e.to_string())?;

    // Verify the master password
    let mut stmt = conn.prepare(
        "SELECT id FROM MasterPassword WHERE password = ?1",
    )
    .map_err(|e| e.to_string())?;

    let master_password_ids: Vec<i64> = stmt.query_map([master_password], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    
    if master_password_ids.is_empty() {
        return Err("Incorrect master password".to_string());
    }

    // Assuming master_password_id is unique and correctly referenced in the user table
    let master_password_id = master_password_ids[0];

    // Query the user-generated table
    let sql = format!("SELECT * FROM {} WHERE master_password = {}", vault_name, master_password_id);
    let mut stmt = conn.prepare(&sql)
        .map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map([master_password_id], |row| {
        // Map the row into your data structure
    Ok(VaultEntry {
        id: row.get(0)?,
        title: row.get(1)?,
        url: row.get(2)?,
        username: row.get(3)?,
        email: row.get(4)?,
        password: row.get(5)?,
        })
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<VaultEntry>, _>>()
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

fn main() {
    // Connect to or create a new database
    let _ = Database::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            add_password,
            add_vault,
            list_vaults,
            read_passwords,
            select_vault,
            authorize
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
