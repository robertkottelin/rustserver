use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use actix_files::Files;

#[derive(Debug, Deserialize, Serialize)]
struct Text {
    id: i32,
    content: String,
}

// ERROR HERE, NOT RIGHT TYPE OF DATA FROM FRONTEND TO DB
#[post("/submit_data")]
async fn submit_data(conn: web::Data<Arc<Mutex<Connection>>>, text: web::Json<Text>) -> impl Responder {
    let conn = conn.lock().unwrap();
    let mut stmt = conn
        .prepare("INSERT INTO texts (content) VALUES (?2)")
        .unwrap();
    stmt.execute(params![text.content]).unwrap();
    HttpResponse::Created().finish()
}

#[get("/get_data")]
async fn get_data(conn: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, content FROM texts").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(Text {
            id: row.get(0)?,
            content: row.get(1)?,
        })
    }).unwrap();
    
    let mut texts = Vec::new();
    for text_result in rows {
        texts.push(text_result.unwrap());
    }

    HttpResponse::Ok().json(texts)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set up the database
    let conn = Arc::new(Mutex::new(Connection::open("src/data.db").unwrap()));


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(submit_data)
            .service(get_data)
            .service(Files::new("/", "./static").index_file("index.html")) // Serve static files

    })

    .bind("127.0.0.1:8080")?
    .run()
    .await
}
