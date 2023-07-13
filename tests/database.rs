use noted::{Database, Note};

fn create_note(id: i32, name: &str, text: &str, completed: bool) -> Note {
    Note {
        id,
        name: name.to_owned(),
        text: text.to_owned(),
        completed,
    }
}

#[test]
fn empty_notes_contain_no_elements() {
    let db = Database::new_in_memory();

    assert!(db.get_all_notes().is_empty())
}

#[test]
fn adding_and_removing_note() {
    let db = Database::new_in_memory();

    db.add_note("test", "testing functionality");
    let first_note = &db.get_all_notes()[0];

    db.remove_note_by_id(first_note.id);
    assert!(db.get_all_notes().is_empty())
}

#[test]
fn adding_one_is_stored_correctly() {
    let db = Database::new_in_memory();

    db.add_note("test", "testing functionality");
    let first_note = &db.get_all_notes()[0];
    assert_eq!(
        &create_note(1, "test", "testing functionality", false),
        first_note
    )
}
