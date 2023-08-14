// use alloc::task;
use sqlx::{Sqlite, SqlitePool};
use std::{error::Error, io, num::ParseIntError, str::FromStr, string::ParseError};
use tokio;

const DATABASE_URL: &str = "sqlite:todos.db";

enum TaskOption {
    Create = 1,
    Read = 2,
    Update = 3,
    Delete = 4,
}

impl FromStr for TaskOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let value = s.parse::<i64>()?;
        // if value == 1 {
        //     return Ok( Self::Create);
        // }
        println!("VALUE {}", s);
        match s.trim() {
            "1" => Ok(Self::Create),
            "2" => Ok(Self::Read),
            "3" => Ok(Self::Update),
            "4" => Ok(Self::Delete),
            _ => Err(()),
        }
        // todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");
    let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
    // SqlitePool::dat
    // if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {}
    let mut user_input = String::new();

    loop {
        println!("Enter a value");
        match io::stdin().read_line(&mut user_input) {
            Ok(bytes_read) => {
                println!("TOTAL BYTES READ : {}", bytes_read);
                println!(
                    "The Value you entered is {} {}",
                    user_input,
                    user_input.len()
                );

                let input_value = user_input.parse::<TaskOption>().unwrap();

                match input_value {
                    TaskOption::Create => {
                        create_task(&pool, "THis is a body not a description".to_string()).await;
                    }
                    TaskOption::Read => {
                        let all_tasks =  get_tasks(&pool).await?;

                        for task in all_tasks {
                            println!("Task {:?}", task);
                        }
                    }
                    TaskOption::Update => todo!(),
                    TaskOption::Delete => todo!(),
                }
            }
            Err(_) => println!("Something went wrong"),
        }
        user_input.clear();
    }
}

async fn create_task(pool: &SqlitePool, description: String) -> Result<i64, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
        INSERT INTO tasks (title, body, done)
        VALUES ( ?1, ?2 , ?3)
        "#,
        "This is a titlte",
        description,
        false,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn get_tasks(pool: &SqlitePool) -> Result<Vec<Task>, Box<dyn Error>> {
    // let mut conn = pool.acquire().await?;

    let recs = sqlx::query!("SELECT * FROM tasks").fetch_all(pool).await?;

    let tasks = recs
        .iter()
        .map(|row| Task {
            id: row.id,
            title: row.title.to_string(),
            description: row.body.to_string(),
            done: row.done,
        })
        .collect();

    Ok(tasks)
}

#[derive(Debug)]
struct Task {
    id: i64,
    title: String,
    description: String,
    done: bool,
}
