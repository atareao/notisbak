use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Note{
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNote{
    pub title: String,
}

impl Note{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<Note>, Error>{
        let notes = query_as!(Note, r#"SELECT id, title, body, created_at, updated_at FROM notes"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(notes)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<Note, Error>{
        let note = query_as!(Note, r#"SELECT id, title, body, created_at, updated_at FROM notes WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(note)
    }

    pub async fn new(pool: web::Data<SqlitePool>, title: &str) -> Result<Note, Error>{
        let body = "";
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
        let id = query("INSERT INTO notes (title, body, created_at, updated_at) VALUES (?, ?, ?, ?);")
            .bind(title)
            .bind(body)
            .bind(created_at)
            .bind(updated_at)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Ok(Self::get(pool, id).await?)
    }
}
