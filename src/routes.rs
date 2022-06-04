use actix_web::{get, post, put, delete, web, Error, HttpResponse, http::StatusCode};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::note::{Note, NewNote};
use crate::category::{Category, NewCategory};
use crate::label::{Label, NewLabel};

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

 #[get("/notes}")]
 pub async fn read_all_note(pool: web::Data<SqlitePool>)->Result<HttpResponse, Error>{
     println!("=== Aqui ===");
     Ok(Note::all(pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/notes/{note_id}")]
pub async fn read_note(pool: web::Data<SqlitePool>, path: web::Path<i64>)->Result<HttpResponse, Error>{
    let note_id = path.into_inner();
    println!("=== {} ===", note_id);
    Ok(Note::get(pool, note_id)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/notes")]
pub async fn create_note(pool: web::Data<SqlitePool>, data: web::Json<NewNote>) -> Result<HttpResponse, Error>{
    Ok(Note::new(pool, &data.into_inner().title, None)
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

#[delete("/notes/{note_id}")]
pub async fn delete_note(pool: web::Data<SqlitePool>, path: web::Path<i64>)->Result<HttpResponse, Error>{
    let note_id = path.into_inner();
    Ok(Note::delete(pool, note_id)
       .await
       .map(|message| HttpResponse::Ok().body(message))
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
