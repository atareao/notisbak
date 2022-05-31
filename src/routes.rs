use actix_web::{get, post, web, Error, HttpResponse, http::StatusCode};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::joke::Note;

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

#[get("/notes")]
pub async fn all_notes(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Note::all(pool)
       .await
       .map(|some_notes| HttpResponse::Ok().json(some_notes))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

