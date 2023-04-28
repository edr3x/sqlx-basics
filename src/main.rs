use std::env;
use std::error::Error;

use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(
        Book,
        "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)",
        book.title,
        book.author,
        book.isbn
    )
    .execute(pool)
    .await
    .expect("\n\nFailed to write to the database\n\n");

    Ok(())
}

async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(
        Book,
        "UPDATE book SET title = $1, author = $2 WHERE isbn = $3",
        book.title,
        book.author,
        isbn
    )
    .execute(pool)
    .await
    .expect("Failed to update the database");

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let book = sqlx::query_as!(Book, "SELECT title, author, isbn FROM book")
        .fetch_all(conn) // to get one we can use fetch_one()
        .await?;

    Ok(book)
}

async fn delete(isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(Book, "DELETE FROM book WHERE isbn = $1", isbn)
        .execute(pool)
        .await?;

    // or
    // sqlx::query!("DELETE FROM book WHERE isbn = $1", isbn)
    //     .execute(pool)
    //     .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("can't find .env file");
    // 1) Create a connection pool
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(postgres_url.as_str()).await?;

    // 2) Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 3) Insert a data
    let book = Book {
        title: "Obi Wan KEbniiijjnk:w".to_string(),
        author: "555Kenobi".to_string(),
        isbn: "921-0751741281".to_string(),
    };
    create(&book, &pool).await?;

    // 4) Update the data
    let book = Book {
        title: "The Fellowship of the RIng".to_string(),
        author: "J.R.R. Tolkien".to_string(),
        isbn: "978-0618640157".to_string(),
    };
    update(&book, &book.isbn, &pool).await?;

    // 5) Read the data
    let books = read(&pool).await?;
    println!("{:?}", books);

    // 6) Delete the data
    delete("921-0751741281", &pool).await?;

    Ok(())
}
