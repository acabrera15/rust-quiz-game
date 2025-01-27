use rusqlite::{Connection, Result};

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

// TODO: add question

// TODO: delete question

// TODO: update question

// TODO: list questions

fn main() -> Result<()> {
    init_db()?;
    Ok(())
}
