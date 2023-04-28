use std::error::Error;

use super::models::Book;

pub async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
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

pub async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
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

pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let book = sqlx::query_as!(Book, "SELECT title, author, isbn FROM book")
        .fetch_all(conn) // to get one we can use fetch_one()
        .await?;

    Ok(book)
}

pub async fn delete(isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(Book, "DELETE FROM book WHERE isbn = $1", isbn)
        .execute(pool)
        .await?;

    // or
    // sqlx::query!("DELETE FROM book WHERE isbn = $1", isbn)
    //     .execute(pool)
    //     .await?;

    Ok(())
}
