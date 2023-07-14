use derive_more::{Display, Error};
use noted::Database;

fn main() -> Result<(), NoteError> {
    let command = std::env::args().skip(1).collect::<Vec<_>>();

    let db = Database::new();

    if command.is_empty() {
        interactive(&db);
        return Ok(());
    }

    handle_command(command, &db)?;
    Ok(())
}

fn handle_command(command: Vec<String>, db: &Database) -> Result<(), NoteError> {
    match command[0].to_lowercase().as_str() {
        "list" | "l" => list_all(db),
        "help" | "h" => help(),
        "add" | "a" => add(db, &command)?,
        "complete" | "c" => call_with_id(db, Database::complete_a_note, &command)?,
        "uncomplete" | "u" => call_with_id(db, Database::uncomplete_a_note, &command)?,
        "delete" | "d" => call_with_id(db, Database::remove_note, &command)?,
        "rename" | "r" => call_with_id_and_string_arg(db, Database::rename_note, &command)?,
        "text" | "t" => call_with_id_and_string_arg(db, Database::change_note_text, &command)?,
        _ => usage(),
    };
    Ok(())
}

fn interactive(db: &Database) {
    println!("Running in interactive mode, enter empty to exit!");
    loop {
        print!("> ");
        let line: String = text_io::read!("{}\n");
        let parsed = parse_string(line);
        if parsed.is_empty() {
            return;
        }

        if let Err(err) = handle_command(parsed, db) {
            println!("Error: {}", err);
        }
    }
}

fn parse_id(s: &str) -> Result<i32, NoteError> {
    s.parse::<i32>().map_err(|_| NoteError::NotAValidID)
}

fn parse_string(s: String) -> Vec<String> {
    let mut between_quotes = false;
    let mut string_buffer = String::new();
    let mut formatted = vec![];
    for c in s.chars() {
        match (c, between_quotes) {
            ('\'', true) | ('\"', true) => {
                between_quotes = false;
                if string_buffer.is_empty() {
                    continue;
                }
                formatted.push(string_buffer);
                string_buffer = String::new();
            }
            ('\'', false) | ('\"', false) => between_quotes = true,
            (' ', false) => {
                if string_buffer.is_empty() {
                    continue;
                }
                formatted.push(string_buffer);
                string_buffer = String::new();
            }
            (_, _) => string_buffer.push(c),
        }
    }
    if !string_buffer.is_empty() {
        formatted.push(string_buffer)
    }
    formatted
}

fn usage() {
    println!("Usage: noted [command]. To display help, use noted help.")
}

fn call_with_id(
    db: &Database,
    function: fn(&Database, i32),
    command: &[String],
) -> Result<(), NoteError> {
    let id_string = command.get(1).ok_or(NoteError::MissingField {
        field: "id".to_owned(),
    })?;
    let id = parse_id(id_string)?;
    function(db, id);
    Ok(())
}

fn call_with_id_and_string_arg(
    db: &Database,
    function: fn(&Database, i32, &str),
    command: &[String],
) -> Result<(), NoteError> {
    let id_string = command.get(1).ok_or(NoteError::MissingField {
        field: "id".to_owned(),
    })?;
    let id = parse_id(id_string)?;

    let str_arg = command.get(2).ok_or(NoteError::MissingField {
        field: "text".to_owned(),
    })?;
    function(db, id, str_arg);
    Ok(())
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
    rename, r       rename a given note, example: noted rename 1 \"new name\"
    text, t         change note text, example: noted text 1 \"new text\"
    "
    )
}

#[derive(Debug, Error, Display)]
enum NoteError {
    #[display(fmt = "Missing field: {}", field)]
    MissingField { field: String },
    #[display(fmt = "The given ID isn't valid!")]
    NotAValidID,
}

#[cfg(test)]
mod test {
    use super::parse_string;

    fn create_string_vector(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn empty_parsed() {
        let parse = String::from("");
        let parsed = parse_string(parse);
        let model = create_string_vector(vec![]);
        assert_eq!(parsed, model)
    }
    #[test]
    fn parse_string_without_quotes() {
        let parse = String::from("no quotations here");
        let parsed = parse_string(parse);
        let model = create_string_vector(vec!["no", "quotations", "here"]);
        assert_eq!(parsed, model)
    }

    #[test]
    fn parse_string_with_single_quotes() {
        let parse = String::from("some 'quotes here'");
        let parsed = parse_string(parse);
        let model = create_string_vector(vec!["some", "quotes here"]);
        assert_eq!(parsed, model)
    }

    #[test]
    fn start_and_end_quotes() {
        let parse = String::from("'please parse ' ' this sentence'");
        let parsed = parse_string(parse);
        let model = create_string_vector(vec!["please parse ", " this sentence"]);
        assert_eq!(parsed, model)
    }

    #[test]
    fn many_whitespace() {
        let parse = String::from("     ");
        let parsed = parse_string(parse);
        let model = create_string_vector(vec![]);
        assert_eq!(parsed, model)
    }

    #[test]
    fn many_quotations() {
        let parse = String::from("''''''");
        let parsed = parse_string(parse);
        let model = create_string_vector(vec![]);
        assert_eq!(parsed, model)
    }
}
