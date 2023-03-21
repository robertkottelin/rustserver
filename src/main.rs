use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePool;
use std::env;
use futures_util::stream::TryStreamExt;


async fn submit_text(pool: web::Data<SqlitePool>, text: web::Json<String>) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO texts (content) VALUES (?);
        "#,
    )
    .bind(&text.0)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Text added successfully."),
        Err(e) => {
            eprintln!("Error while adding text: {}", e);
            HttpResponse::InternalServerError().body("Failed to add text.")
        }
    }
}

async fn get_texts(pool: web::Data<SqlitePool>) -> impl Responder {
    let mut texts = Vec::new();
    let mut rows = sqlx::query("SELECT content FROM texts")
        .fetch(pool.as_ref());

    while let Some(row) = rows.try_next().await.unwrap() {
        texts.push(row.get("content"));
    }

    HttpResponse::Ok().json(texts)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set up the database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create connection pool");

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/submit_text", web::post().to(submit_text))
            .route("/get_texts", web::get().to(get_texts))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}