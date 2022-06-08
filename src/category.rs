use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Category{
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
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
        Self::get(pool, id).await
    }

    pub async fn update(pool: web::Data<SqlitePool>, category: Category) -> Result<Category, Error>{
        query("UPDATE categories SET name=? WHERE id=?;")
            .bind(label.name)
            .bind(label.id)
            .execute(pool.get_ref())
            .await?;
        Self::get(pool, label.id).await
    }

    pub async fn delete(pool: web::Data<SqlitePool>, id: i64) -> Result<String, Error>{
        query("DELETE FROM categories WHERE id = ?;")
            .bind(id)
            .execute(pool.get_ref())
            .await;
        Ok("Category deleted".to_string())
    }
}
