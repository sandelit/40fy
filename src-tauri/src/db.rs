use regex::Regex;
use rusqlite::{params, Connection};
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct VaultEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub master_password_id: String,
    pub created_at_date_time: String,
    pub updated_at_date_time: String,
}

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Database, rusqlite::Error> {
        let conn = Connection::open("./passwords.db")?;
        let db = Database { conn };
        db.create_table();
        Ok(db)
    }

    fn create_table(&self) -> Result<(), rusqlite::Error> {
        println!("Creating master password table");
        self.conn.execute("PRAGMA foreign_keys", params![])?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS MasterPassword(
                id TEXT PRIMARY KEY,
                password TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    fn is_valid_table_name(name: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
        re.is_match(name)
    }

    pub fn connection() -> Result<Connection, String> {
        let conn = rusqlite::Connection::open("./passwords.db").map_err(|e| e.to_string())?;
        return Ok(conn);
    }

    pub fn add_vault_entry(
        &self,
        vault: String,
        title: String,
        url: String,
        username: String,
        email: String,
        password: String,
        master_password_id: String,
    ) -> Result<(), String> {
        let sql = format!(
        "INSERT INTO {} (id, master_password_id, title, url, username, email, password, createdAtDateTime, lastUpdatedAtDateTime) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'))",
        vault
    );

        let id = Uuid::new_v4().to_string();
        self.conn
            .execute(
                &sql,
                [
                    id,
                    master_password_id,
                    title,
                    url,
                    username,
                    email,
                    password,
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn read_entries(&self, vault: &str) -> Result<Vec<VaultEntry>, String> {
        if !Database::is_valid_table_name(vault) {
            return Err("Invalid table name".to_string());
        }

        let sql = format!("SELECT * FROM {}", vault);
        let mut stmt = self.conn.prepare(&sql).map_err(|e| e.to_string())?;

        let entry_iter = stmt
            .query_map([], |row| {
                Ok(VaultEntry {
                    id: row.get(0)?,
                    master_password_id: row.get(1)?,
                    title: row.get(2)?,
                    url: row.get(3)?,
                    username: row.get(4)?,
                    email: row.get(5)?,
                    password: row.get(6)?,
                    created_at_date_time: row.get(7)?,
                    updated_at_date_time: row.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let entries: Result<Vec<VaultEntry>, _> = entry_iter.collect();
        entries.map_err(|e| e.to_string())
    }

    pub fn list_vaults(&self) -> Result<Vec<String>, String> {
        // Prepare the SQL query to select all user-created table names
        let mut statement = self.conn
        .prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'MasterPassword'",
        )
        .map_err(|e| e.to_string())?;

        // Execute the query and collect table names into a Vec<String>
        let tables = statement
            .query_map([], |row| Ok(row.get::<_, String>(0)?))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(tables)
    }

    pub fn add_vault(&self, name: &str, password: &str) -> Result<(), String> {
        let conn = rusqlite::Connection::open("./passwords.db").map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS MasterPassword(
                id TEXT PRIMARY KEY,
                password TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4().to_string();
        // Insert into master table and get the ID from it
        conn.execute(
            "INSERT INTO MasterPassword (password, id) VALUES (?1, ?2)",
            params![password, id],
        )
        .map_err(|e| e.to_string())?;

        //Create new user-generated table using the master_password_id as a foreign key
        let statement = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id TEXT PRIMARY KEY,
            master_password_id TEXT,
            title TEXT NOT NULL,
            url TEXT,
            username TEXT,
            email TEXT,
            password TEXT,
            createdAtDateTime TEXT NOT NULL,
            lastUpdatedAtDateTime TEXT NOT NULL,
            FOREIGN KEY(master_password_id) REFERENCES MasterPassword(id)
        )",
            name
        );

        conn.execute(&statement, rusqlite::params![])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn select_vault(&self, name: &str, master_password: &str) -> Result<String, String> {
        // Attempt to verify the master password and get its UUID
        let result = self.conn.query_row(
            "SELECT id FROM MasterPassword WHERE password = ?1",
            rusqlite::params![master_password],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(master_password_id) => Ok(master_password_id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err("Incorrect master password".to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
