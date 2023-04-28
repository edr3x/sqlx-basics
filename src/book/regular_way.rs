use super::models::Book;
use std::error::Error;

pub async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(isbn)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";

    // NOTE: 1) using fetch_one

    // let query = sqlx::query(q);
    // let row = query.fetch_one(conn).await?;
    // let book = Book {
    //     title: row.get("title"),
    //     author: row.get("author"),
    //     isbn: row.get("isbn"),
    // };

    // NOTE: 2)  using fetch_optional (returns NONE insted of error if not found)

    // let query = sqlx::query(q);
    // let maybe_row = query.fetch_optional(conn).await?;
    // let book = maybe_row.map(|row| Book {
    //     title: row.get("title"),
    //     author: row.get("author"),
    //     isbn: row.get("isbn"),
    // });

    // NOTE: 3) using fetch_all (returns a vector of rows)

    // let query = sqlx::query(q);
    // let rows = query.fetch_all(conn).await?;
    // let books: Vec<Book> = rows
    //     .iter()
    //     .map(|row| Book {
    //         title: row.get("title"),
    //         author: row.get("author"),
    //         isbn: row.get("isbn"),
    //     })
    //     .collect();

    // NOTE: 4) using fetch (this returns async stream so it is better for large data sets)

    // let query = sqlx::query(q);
    // let mut rows = query.fetch(conn);
    // let mut books = vec![];

    // // TODO: here `.try_next()` requrires a future crate
    // // futures = "0.3"
    // // use futures::TryStreamExt;
    // while let Some(row) = rows.try_next().await? {
    //     books.push(Book {
    //         title: row.get("title"),
    //         author: row.get("author"),
    //         isbn: row.get("isbn"),
    //     });
    // }

    // NOTE: 6) Turning those returned rows into concrede types
    // this way we don't have to type many boilerplates

    let query_as = sqlx::query_as::<_, Book>(q);

    let books = query_as.fetch_all(conn).await?;

    Ok(books)
}

pub async fn delete(isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "DELETE FROM book WHERE isbn = $1";

    sqlx::query(query).bind(isbn).execute(pool).await?;

    Ok(())
}

pub async fn transaction(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let mut txn = pool.begin().await?;

    let result = sqlx::query("").execute(&mut txn).await?;

    // NOTE: if the query did not affect any rows then rollback the transaction
    if result.rows_affected() != 1 {
        txn.rollback().await?; // rollback the transaction if the query is not successful
        return Err("failed to insert row".into());
    }

    let result = sqlx::query("").execute(&mut txn).await?;

    // NOTE: if the query did not affect any rows then rollback the transaction
    if result.rows_affected() != 1 {
        txn.rollback().await?; // rollback whole transaction if this fails i.e. even if the first
                               // one was success
        return Err("failed to insert row".into());
    }

    // NOTE: if performaing a query that effects multiple rows we can do this
    let name = "John";
    let query = "DELETE FROM book WHERE author_name = $1";

    let result = sqlx::query(query).bind(name).execute(&mut txn).await?;

    if result.rows_affected() == 0 {
        // if no rows were affected then rollback the transaction
        txn.rollback().await?;
        return Err("failed to delete row".into());
    }

    // NOTE: if everything is successful then commit the transaction
    txn.commit().await?;

    Ok(())
}
