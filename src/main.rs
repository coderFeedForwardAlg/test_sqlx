use sqlx::{prelude::FromRow, Row};
use std::error::Error;

#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn add_book_api(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    print!("book api called");
    let book = book_builder(
        &String::from("book1"),
        &String::from("me"),
        &String::from("000"),
    );
    add_book(&book, pool).await?;
    Ok(())
}

fn book_builder(title: &String, author: &String, isbn: &String) -> Book {
    Book {
        title: title.to_string(),
        author: author.to_string(),
        isbn: isbn.to_string(),
    }
}

async fn change_book(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn add_book(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES  ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_books(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query_as::<_, Book>(q);

    let books = query.fetch_all(pool).await?;
    Ok(books)
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5431/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let updated_book = Book {
        title: "a cool book".to_string(),
        author: "sartre".to_string(),
        isbn: "".to_string(),
    };

    /* change_book(&updated_book, "100-000-000", &pool).await?; */
    /* add_book_api(&pool).await?; */
    let books = get_books(&pool).await?;
    books.iter().for_each(|book| {
        println!("{}", book.title);
    });
    Ok(())
}
