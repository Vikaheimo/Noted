use rusqlite::{named_params, Connection};
use std::path::Path;

#[derive(Debug)]
pub struct Database {
    db: Connection,
}

const DATABASE_PATH: &str = "db.db";

impl Default for Database {
    fn default() -> Self {
        Database::new_in_memory()
    }
}

impl Database {
    pub fn new() -> Self {
        let first_time = !Path::new(DATABASE_PATH).exists();
        let db = Database {
            db: Connection::open("db.db").unwrap(),
        };

        if first_time {
            db.create_tables();
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
                completed   INTEGER NOT NULL
            )",
                (),
            )
            .unwrap();
    }

    pub fn add_note(&self, name: &str, text: &str) {
        self.db
            .execute(
                "INSERT INTO notes (name, text, completed) VALUES (?1, ?2, FALSE)",
                (name, text),
            )
            .unwrap();
    }

    pub fn search_notes(&self, name: &str) -> Vec<Note> {
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

    pub fn remove_note(&self, id: i32) {
        self.db
            .execute(
                "DELETE FROM notes WHERE id = :id",
                named_params! {":id": id},
            )
            .unwrap();
    }

    pub fn complete_a_note(&self, id: i32) {
        self.db
            .execute(
                "UPDATE notes SET completed = TRUE WHERE id = :id",
                named_params! {":id": id},
            )
            .unwrap();
    }

    pub fn rename_note(&self, id: i32, new_name: &str) {
        self.db
            .execute(
                "UPDATE notes SET name = :name WHERE id = :id",
                named_params! {":name": new_name, ":id": id},
            )
            .unwrap();
    }

    pub fn change_note_text(&self, id: i32, new_text: &str) {
        self.db
            .execute(
                "UPDATE notes SET text = :text WHERE id = :id",
                named_params! {":text": new_text, ":id": id},
            )
            .unwrap();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub completed: bool,
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

impl Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.completed, self.id, &self.name, &self.text).cmp(&(
            other.completed,
            other.id,
            &other.name,
            &other.text,
        ))
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.completed.partial_cmp(&other.completed) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.text.partial_cmp(&other.text)
    }
}

#[cfg(test)]
mod test {
    use super::Note;

    fn create_note(id: i32, name: &str, text: &str, completed: bool) -> Note {
        Note {
            id,
            name: name.to_owned(),
            text: text.to_owned(),
            completed,
        }
    }

    #[test]
    fn notes_sorting() {
        let mut notes = vec![
            create_note(1, "test", "test", true),
            create_note(2, "test", "test", false),
            create_note(3, "test", "test", true),
            create_note(4, "test", "test", false),
        ];

        let model = vec![
            create_note(2, "test", "test", false),
            create_note(4, "test", "test", false),
            create_note(1, "test", "test", true),
            create_note(3, "test", "test", true),
        ];

        notes.sort_unstable();

        assert_eq!(notes, model)
    }
}
