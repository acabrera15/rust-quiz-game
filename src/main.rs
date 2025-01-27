use rusqlite::{params, Connection, Result};

fn init_db() -> Result<()> {
    let path = "quiz_db.db3";

    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY,
            question TEXT NOT NULL
            )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS options (
            id INTEGER PRIMARY KEY,
            question_id INTEGER NOT NULL,
            option_text TEXT NOT NULL,
            is_correct BOOLEAN NOT NULL,
            FOREIGN KEY(question_id) REFERENCES questions(id)
        )",
        [],
    )?;

    Ok(())
}

fn add_question(conn: &Connection, question: &str, options: Vec<(&str, bool)>) -> Result<()> {
    // insert question
    conn.execute("INSERT INTO questions (question) VALUES (?1)", [question])?;
    let last_id = conn.last_insert_rowid();

    for (option_text, correct) in options {
        conn.execute(
            "INSERT INTO options (question_id, option_text, is_correct) VALUES (?1, ?2, ?3)",
            params![last_id, option_text, correct],
        )?;
    }

    Ok(())
}

// TODO: add question

// TODO: delete question

// TODO: update question

// TODO: list questions

fn main() -> Result<()> {
    init_db()?;
    let path = "quiz_db.db3";
    let conn = Connection::open(path);

    println!("Welcome to admin side");
    println!("Here you can manipulate the questions to be shown to users");
    println!("Select one of the options below");
    println!("-------------------------------");
    println!("1. List questions");
    println!("2. Add question");
    println!("3. Update question");
    println!("4. Delete question");

    Ok(())
}
