use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCategory{
    pub id: i64,
    pub note_id: i64,
    pub category_id: i64,
}

impl NoteCategory{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<NoteCategory>, Error>{
        let notes_categories = query_as!(NoteCategory, r#"SELECT id, note_id, category_id FROM notes_categories"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(notes_categories)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<NoteCategory, Error>{
        let note_category = query_as!(NoteCategory, r#"SELECT id, note_id, category_id FROM notes_categories WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(note_category)
    }

    pub async fn new(pool: web::Data<SqlitePool>, note_id: i64, category_id: i64) -> Result<NoteCategory, Error>{
        let id = query("INSERT INTO notes_categories (note_id, category_id) VALUES (?, ?);")
            .bind(note_id)
            .bind(category_id)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Ok(Self::get(pool, id).await?)
    }
}

