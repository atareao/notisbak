mod note;
mod label;
mod category;
mod note_label;

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
            .data(pool.clone())
            .service(root)
            .service(all_jokes)
            .service(get_joke)
            .service(get_random_joke)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
