use std::error::Error;

use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
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

async fn delete(isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "DELETE FROM book WHERE isbn = $1";

    sqlx::query(query).bind(isbn).execute(pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1) Create a connection pool
    let postgres_url = "postgresql://postgres:pass@localhost:5432/sqlxlearn";
    let pool = sqlx::postgres::PgPool::connect(postgres_url).await?;

    // 2) Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 3) Insert a book
    // let book = Book {
    //     title: "Harry Potter".to_string(),
    //     author: "J.K Rowling".to_string(),
    //     isbn: "978-0618740187".to_string(),
    // };
    // create(&book, &pool).await?;

    // 4) Update the data
    // let book = Book {
    //     title: "The Fellowship of the finggggg".to_string(),
    //     author: "J.R.R. Tolkien".to_string(),
    //     isbn: "978-0618640157".to_string(),
    // };

    // update(&book, &book.isbn, &pool).await?;

    // 5) Read the data
    let books = read(&pool).await?;
    println!("{:?}", books);

    Ok(())
}
