use std::path::Path;

use rusqlite::{params, Connection};

#[derive(Debug, serde::Serialize)]
pub struct Password {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Password {
    pub fn new(
        id: usize,
        title: String,
        url: String,
        username: String,
        email: String,
        password: String,
    ) -> Password {
        Password {
            id,
            title,
            url,
            username,
            email,
            password,
        }
    }
}

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn connect() -> Result<Connection, rusqlite::Error> {
        return Ok(Connection::open("./passwords.db")?)
    }

    pub fn init() -> Result<Database, rusqlite::Error> {
        let conn = Connection::open("./passwords.db")?;
        let db = Database { conn };

        db.create_table();

        Ok(db)
    }

    fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute("PRAGMA foreign_keys", params![]);
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS MasterPassword(
                id INTEGER PRIMARY KEY,
                password TEXT NOT NULL
            )",
        [])?;

        Ok(())
    }
}

/*
impl Database {
    pub fn new(key: String) -> Result<Database, rusqlite::Error> {
        let conn = Connection::open("passwords.db")?;
        // set password to our database. without this passphrase database is not readable
        //conn.pragma_update(Some(DatabaseName::Main), "KEY", key)?;
        let db = Database { conn };
        db.create_table()?;
        Ok(db)
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS passwords(
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    url TEXT,
                    username TEXT,
                    email TEXT,
                    password TEXT NOT NULL
                )
            ",
        )?;
        Ok(())
    }

    pub fn load(&self) -> Vec<Password> {
        let mut statement = self.conn.prepare("select * from passwords").unwrap();
        let items: Vec<Password> = statement
            .query_map([], |row| {
                let password = Password::new_with_id(
                    row.get("id").unwrap(),
                    row.get("title").unwrap(),
                    row.get("url").unwrap(),
                    row.get("username").unwrap(),
                    row.get("email").unwrap(),
                    row.get("password").unwrap(),
                );
                Ok(password)
            })
            .unwrap()
            .map(|i| i.unwrap())
            .collect();
        items
    }

    pub fn insert(&self, password: &Password) {
        self.conn
            .execute(
                "insert into passwords (title, username, password) values (?1, ?2, ?3)",
                params![password.title, password.username, password.password],
            )
            .unwrap();
    }

    pub fn update(&self, id: usize, password: &Password) {
        self.conn
            .execute(
                "update passwords set title=?1, username=?2, password=?3 where id=?4",
                params![password.title, password.username, password.password, id],
            )
            .unwrap();
    }

    pub fn delete(&self, id: usize) {
        self.conn
            .execute("delete from passwords where id=?1", params![id])
            .unwrap();
    }
}
 */
