use rusqlite::{named_params, Connection};
use std::path::Path;

pub struct Database {
    db: Connection,
}

const DATABASE_PATH: &'static str = "db.db";

impl Database {
    pub fn new() -> Self {
        let db = Database {
            db: Connection::open("db.db").unwrap(),
        };

        if !Path::new(DATABASE_PATH).exists() {
            db.create_tables()
        }
        db
    }

    pub fn new_in_memory() -> Self {
        let db = Database {
            db: Connection::open_in_memory().unwrap(),
        };
        db.create_tables();
        db
    }

    fn create_tables(&self) {
        self.db
            .execute(
                "CREATE TABLE notes (
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                text        TEXT,
                completed   INTEGER NOT NULL,
            )",
                (),
            )
            .unwrap();
    }

    pub fn add_note(&self, name: String, text: String) {
        self.db
            .execute("INSERT INTO notes (name, text, completed)", (name, text, 0))
            .unwrap();
    }

    pub fn search_notes(&self, name: String) -> Vec<Note> {
        let mut statement = self
            .db
            .prepare("SELECT * FROM notes WHERE name Like :name")
            .unwrap();
        statement
            .query_map(named_params! {":name": name}, Note::from_row)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    pub fn get_all_notes(&self) -> Vec<Note> {
        let mut statement = self.db.prepare("SELECT * FROM notes").unwrap();
        statement
            .query_map((), Note::from_row)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Note {
    id: i32,
    name: String,
    text: String,
    completed: bool,
}

impl Note {
    pub fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Note {
            id: row.get(0)?,
            name: row.get(1)?,
            text: row.get(2)?,
            completed: row.get(3)?,
        })
    }
}
