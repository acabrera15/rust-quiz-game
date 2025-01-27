use core::num;
use rusqlite::{params, Connection, Result};
use std::io;

struct Option {
    option: String,
    is_correct: bool,
}
struct Question {
    id: u32,
    question: String,
    options: Vec<Option>,
}

fn init_db() -> Result<()> {
    let path = "quiz_db.db3";
    let conn = Connection::open(path)?;

    // create table question table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY,
            question TEXT NOT NULL
            )",
        [],
    )?;

    // create options table
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

// TODO: list questions
fn list_questions(conn: &Connection) -> Result<()> {
    let mut questions_vec: Vec<Question> = Vec::new();

    // get all questions
    let mut stmt = conn.prepare("SELECT * FROM questions")?;
    let questions_iter = stmt.query_map([], |row| {
        let id: u32 = row.get(0)?;
        let question: String = row.get(1)?;
        Ok((id, question))
    })?;

    // iterate through question geting the options adding them to structs
    for question in questions_iter {
        match question {
            Ok((id, question)) => {
                let mut options_stmt =
                    conn.prepare("SELECT * FROM options WHERE question_id = ?1")?;
                let options_iter = options_stmt.query_map(params![id], |row| {
                    let option_text: String = row.get(2)?;
                    let is_correct: bool = row.get(3)?;
                    Ok((option_text, is_correct))
                })?;

                let mut option_vec: Vec<Option> = Vec::new();

                for option in options_iter {
                    match option {
                        Ok((option_text, is_correct)) => {
                            option_vec.push(Option {
                                option: option_text,
                                is_correct: is_correct,
                            });
                        }
                        Err(err) => println!("Error reading row: {}", err),
                    }
                }

                questions_vec.push(Question {
                    id: id,
                    question: question,
                    options: option_vec,
                });
            }
            Err(err) => println!("Error reading row: {}", err),
        }
    }

    // neatly print out the questions and options
    if questions_vec.len() == 0 {
        println!("\nThere are no questions added! \n")
    } else {
        for question in questions_vec {
            println!("\n{}.{}", question.id, question.question);
            for option in question.options {
                println!("\t{} {}", option.option, option.is_correct);
            }
            println!("\n");
        }
    }

    Ok(())
}

fn add_question(conn: &Connection, question: &str, options: Vec<Option>) -> Result<()> {
    // insert question
    conn.execute("INSERT INTO questions (question) VALUES (?1)", [question])?;
    let last_id = conn.last_insert_rowid();

    for option in options {
        conn.execute(
            "INSERT INTO options (question_id, option_text, is_correct) VALUES (?1, ?2, ?3)",
            params![last_id, option.option, option.is_correct],
        )?;
    }

    Ok(())
}

// TODO: delete question
fn delete_question(conn: &Connection, id: u32) -> Result<()> {
    conn.execute("DELETE FROM questions WHERE id = ?1", [id])?;

    Ok(())
}

// TODO: update question

fn main() -> Result<()> {
    init_db()?;
    let path = "quiz_db.db3";
    let conn = Connection::open(path)?;

    println!("Welcome to admin side");
    println!("Here you can manipulate the questions to be shown to users");

    loop {
        println!("Select one of the options below");
        println!("-------------------------------");
        println!("1. List questions");
        println!("2. Add question");
        println!("3. Update question");
        println!("4. Delete question");

        let mut input = String::new();
        let mut num_input: u8;

        loop {
            input.clear();

            io::stdin()
                .read_line(&mut input)
                .expect("Unable to read line");

            num_input = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("enter a valid number \n");
                    continue;
                }
            };

            if num_input > 4 || num_input < 1 {
                println!("Enter a number from 1-4 \n");
                continue;
            }

            break;
        }

        match num_input {
            1 => {
                if let Err(_) = list_questions(&conn) {
                    println!("Failed to print questions")
                }
            }
            2 => {
                if let Err(err) = add_question(
                    &conn,
                    "How tall is Andrew?",
                    vec![
                        Option {
                            option: String::from("5'8"),
                            is_correct: true,
                        },
                        Option {
                            option: String::from("5'2"),
                            is_correct: false,
                        },
                        Option {
                            option: String::from("5'5"),
                            is_correct: false,
                        },
                    ],
                ) {
                    println!("Failed to add question: {}", err);
                } else {
                    println!("Question added successfully");
                }
            }
            3 => println!("update \n"),
            4 => {
                println!("Enter the id of the question you want to delete");
                let mut user_input = String::new();
                let num_input: u32;

                loop {
                    user_input.clear();
                    io::stdin()
                        .read_line(&mut user_input)
                        .expect("Unable to read line");

                    num_input = match user_input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Enter a valid number");
                            continue;
                        }
                    };
                    break;
                }
                if let Err(err) = delete_question(&conn, num_input) {
                    println!("Failed to delete question: {}", err);
                } else {
                    println!("Question deleted successfully");
                }
            }
            _ => println!("invalid option \n"),
        }
    }

    Ok(())
}
