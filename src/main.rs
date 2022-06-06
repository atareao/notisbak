mod note;
mod label;
mod category;
mod note_label;
mod note_category;
mod routes;

use sqlx::sqlite::SqlitePoolOptions;
use actix_web::{App, HttpServer, web::Data};
use dotenv::dotenv;
use std::env;
use routes::{root,
             all_notes, create_note, read_note, update_note, delete_note,
             all_categories, new_category, all_labels, new_label};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("pool failed");
    HttpServer::new(move ||{
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(root)
            .service(all_notes)
            .service(create_note)
            .service(read_note)
            .service(update_note)
            .service(delete_note)
            .service(all_categories)
            .service(new_category)
            .service(all_labels)
            .service(new_label)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
