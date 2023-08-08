use sqlx::{Sqlite, SqlitePool};
use std::{io, num::ParseIntError, str::FromStr, string::ParseError};
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
async fn main() {
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
                    TaskOption::Read => todo!(),
                    TaskOption::Update => todo!(),
                    TaskOption::Delete => todo!(),
                }
            }
            Err(_) => println!("Something went wrong"),
        }
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
