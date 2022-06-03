use actix_web::{get, post, put, web, Error, HttpResponse, http::StatusCode};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::note::{Note, NewNote};
use crate::category::{Category, NewCategory};
use crate::label::{Label, NewLabel};

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

#[post("/notes")]
pub async fn new_note(pool: web::Data<SqlitePool>, data: web::Json<NewNote>) -> Result<HttpResponse, Error>{
    Ok(Note::new(pool, &data.into_inner().title)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[put("/notes")]
pub async fn update_note(pool: web::Data<SqlitePool>, data: web::Json<Note>) -> Result<HttpResponse, Error>{
    let note = data.into_inner();
    Ok(Note::update(pool, note)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/categories")]
pub async fn all_categories(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Category::all(pool)
       .await
       .map(|some_categories| HttpResponse::Ok().json(some_categories))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/categories")]
pub async fn new_category(pool: web::Data<SqlitePool>, data: web::Json<NewCategory>) -> Result<HttpResponse, Error>{
    Ok(Category::new(pool, &data.into_inner().name)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/labels")]
pub async fn all_labels(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Label::all(pool)
       .await
       .map(|some_labels| HttpResponse::Ok().json(some_labels))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/labels")]
pub async fn new_label(pool: web::Data<SqlitePool>, data: web::Json<NewLabel>) -> Result<HttpResponse, Error>{
    Ok(Label::new(pool, &data.into_inner().name)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
