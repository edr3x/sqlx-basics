use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}
