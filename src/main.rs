use sqlx::{SqlitePool, Sqlite};
use std::io;
use tokio;

const DATABASE_URL: &str = "sqlite:todos.db";

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    // let pool = SqlitePool::connect(DATABASE_URL).await?;
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
            }
            Err(_) => println!("Something went wrong"),
        }
    }
}

async fn create_task(pool: &SqlitePool, description: String) -> Result<i64, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
        INSERT INTO tasks (description)
        VALUES ( ?1 )
        "#,
        description,
    )
    .execute(pool);

    Ok(id)
}
