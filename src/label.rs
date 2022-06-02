use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label{
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLabel{
    pub name: String,
}

impl Label{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<Label>, Error>{
        let labels = query_as!(Label, r#"SELECT id, name FROM labels"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(labels)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<Label, Error>{
        let label = query_as!(Label, r#"SELECT id, name FROM labels WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(label)
    }

    pub async fn new(pool: web::Data<SqlitePool>, name: &str) -> Result<Label, Error>{
        let id = query("INSERT INTO labels (name) VALUES (?);")
            .bind(name)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Ok(Self::get(pool, id).await?)
    }
}

