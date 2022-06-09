use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteLabel{
    pub id: i64,
    pub note_id: i64,
    pub label_id: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewNoteLabel{
    pub note_id: i64,
    pub label_id: i64,
}

impl NoteLabel{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<NoteLabel>, Error>{
        let notes_labels = query_as!(NoteLabel, r#"SELECT id, note_id, label_id FROM notes_labels"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(notes_labels)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<NoteLabel, Error>{
        let note_label = query_as!(NoteLabel, r#"SELECT id, note_id, label_id FROM notes_labels WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(note_label)
    }

    pub async fn new(pool: web::Data<SqlitePool>, note_id: i64, label_id: i64) -> Result<NoteLabel, Error>{
        let id = query("INSERT INTO notes_labels (note_id, label_id) VALUES (?, ?);")
            .bind(note_id)
            .bind(label_id)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Self::get(pool, id).await
    }

    pub async fn update(pool: web::Data<SqlitePool>, note_label: NoteLabel) -> Result<NoteLabel, Error>{
        query("UPDATE notes_labels SET note_id=?, label_id=? WHERE id=?;")
            .bind(note_label.note_id)
            .bind(note_label.label_id)
            .execute(pool.get_ref())
            .await?;
        Self::get(pool, note_label.id).await
    }

    pub async fn delete(pool: web::Data<SqlitePool>, id: i64) -> Result<String, Error>{
        query("DELETE FROM notes_labels WHERE id = ?;")
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        Ok("Note Label deleted".to_string())
    }
}

