use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category{
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCategory{
    pub name: String,
}

impl Category{
    pub async fn all(pool: web::Data<SqlitePool>) -> Result<Vec<Category>, Error>{
        let categories = query_as!(Category, r#"SELECT id, name FROM categories"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(categories)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<Category, Error>{
        let category = query_as!(Category, r#"SELECT id, name FROM categories WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(category)
    }

    pub async fn new(pool: web::Data<SqlitePool>, name: &str) -> Result<Category, Error>{
        let id = query("INSERT INTO categories (name) VALUES (?);")
            .bind(name)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Ok(Self::get(pool, id).await?)
    }
}

