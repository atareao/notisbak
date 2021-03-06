use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use sqlx::{sqlite::{SqlitePool, SqliteQueryResult}, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};
use crate::label::Label;
use utoipa::Component;

//https://github.com/juhaku/utoipa

#[derive(Debug, FromRow, Serialize, Deserialize, Component)]
pub struct Note{
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewNote{
    pub title: String,
}

mod note_api{
    use crate::note::Note;

    #[utoipa::path(
        get,
        path = "/notes/{id}",
        responses(
            (status = 200, description = "Note found succesfully", body = Note),
            (status = 404, description = "Note was not found")
        ),
        params(
            ("id" = i64, path, description = "Note database id to get Note for"),
        )
    )]
    async fn get_note_by_id(note_id: i64) -> Note {
        let current = chrono::Utc::now().naive_utc();
        Note {
            id: note_id,
            title: "Sample title".to_string(),
            body: "Sample body".to_string(),
            created_at: current,
            updated_at: current,
        }
    }
}

impl Note{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<Note>, Error>{
        println!("Get all");
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

    pub async fn new(pool: web::Data<SqlitePool>, title: &str, body_option: Option<&str>) -> Result<Note, Error>{
        let body = body_option.unwrap_or("");
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
        Self::get(pool, id).await
    }

    pub async fn update(pool: web::Data<SqlitePool>, note: Note) -> Result<Note, Error>{
        let updated_at = Utc::now().naive_utc();
        query("UPDATE notes SET title=?, body=?, updated_at=? WHERE id=?;")
            .bind(note.title)
            .bind(note.body)
            .bind(updated_at)
            .bind(note.id)
            .execute(pool.get_ref())
            .await?;
        Self::get(pool, note.id).await
    }

    pub async fn delete(pool: web::Data<SqlitePool>, id: i64) -> Result<String, Error>{
        query("DELETE FROM notes WHERE id = ?;")
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        Ok("Note deleted".to_string())
    }

    pub async fn add_label(self, pool: web::Data<SqlitePool>, label_id: i64) -> Result<SqliteQueryResult, Error>{
        query("INSERT INTO notes_labels (note_id, label_id) VALUES (?, ?);")
            .bind(self.id)
            .bind(label_id)
            .execute(pool.get_ref())
            .await
    }

    pub async fn delete_label(self, pool: web::Data<SqlitePool>, label_id: i64) -> Result<SqliteQueryResult, Error>{
        query("DELETE FROM notes_labels WHERE node_id = ?, label_id = ?")
            .bind(self.id)
            .bind(label_id)
            .execute(pool.get_ref())
            .await
    }

    pub async fn get_labels(self, pool: web::Data<SqlitePool>) -> Result<Vec<Label>, Error>{
        let labels = query_as!(Label, r#"SELECT l.id, l.name FROM labels l INNER JOIN notes_labels nl ON l.id = nl.label_id AND nl.note_id = ?"#, self.id)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(labels)
    }

    pub async fn get_label(self, pool: web::Data<SqlitePool>, label_id: i64) -> Result<Label, Error>{
        let label = query_as!(Label, r#"SELECT id, name FROM labels WHERE id = ?"#, self.id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(label)
    }
}
