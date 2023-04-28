use std::error::Error;

mod book;
use book::models::Book;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("can't find .env file");

    // 1) Create a connection pool
    let postgres_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(postgres_url.as_str()).await?;

    // 2) Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 3) Insert a book
    // let book = Book {
    //     title: "Harry Potter".to_string(),
    //     author: "J.K Rowling".to_string(),
    //     isbn: "978-0618740187".to_string(),
    // };

    // book::regular_way::create(&book, &pool).await?;

    // book::macro_way::create(&book, &pool).await?;

    // 4) Update the data
    // let book = Book {
    //     title: "The Fellowship of the finggggg".to_string(),
    //     author: "J.R.R. Tolkien".to_string(),
    //     isbn: "978-0618640157".to_string(),
    // };

    // book::regular_way::update(&book, &book.isbn, &pool).await?;

    // book::macro_way::update(&book, &book.isbn, &pool).await?;

    // 5) Read the data
    let books = book::regular_way::read(&pool).await?;
    // let books = book::macro_way::read(&pool).await?;
    println!("{:?}", books);

    // 6) Delete the data
    // book::regular_way::delete("921-0751741281", &pool).await?;

    // book::macro_way::delete("921-0751741281", &pool).await?;

    Ok(())
}
