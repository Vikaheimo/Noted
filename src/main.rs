use derive_more::{Display, Error};
use noted::Database;

fn main() -> Result<(), NoteError> {
    let command = std::env::args().skip(1).collect::<Vec<_>>();

    if command.is_empty() {
        usage();
        return Ok(());
    }

    let db = Database::new();

    match command[0].to_lowercase().as_str() {
        "list" | "l" => list_all(&db),
        "help" | "h" => help(),
        "add" | "a" => add(&db, &command)?,
        _ => usage(),
    }

    Ok(())
}

fn usage() {
    println!("Usage: noted [command]. To display help, use noted help.")
}

fn add(db: &Database, command: &[String]) -> Result<(), NoteError> {
    let name = command.get(1).ok_or(NoteError::MissingField {
        field: "Name".to_string(),
    })?;

    let text = command.get(2).map(|s| s.to_owned()).unwrap_or_default();
    db.add_note(name, &text);
    Ok(())
}

fn list_all(db: &Database) {
    println!("Not completed:");
    let mut notes = db.get_all_notes();
    notes.sort_unstable();
    let mut first = true;

    for note in notes {
        if note.completed && first {
            first = false;
            println!("\nCompleted:");
        }
        println!("{:0>3}: {}  |  {}", note.id, note.name, note.text);
    }
}

fn help() {
    usage();
    println!(
        "Commands:

    help, h         shows this help
    add, a          add a new note, example: noted add test \"This is a test note!\"
    list, l         lists all notes,
    complete, c     completes a given taks,
    delete, d       deletes a given task,
    update, u       update a given task with a new name or text
    "
    )
}

#[derive(Debug, Error, Display)]
enum NoteError {
    #[display(fmt = "Missing field: {}", field)]
    MissingField { field: String },
}
