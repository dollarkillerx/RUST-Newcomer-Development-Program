use std::sync::Arc;
use ntex::web::{
    self,
    types::{Json, State,Path},
};

use crate::{errors::CustomError, models::article::Article, AppState};

#[web::get("/article")]
async fn get_articles(state: State<Arc<AppState>>) -> Result<Json<Vec<Article>>, CustomError> {
   let conn = &state.db_pool;
    let articles:Vec<Article> = sqlx::query!("SELECT * FROM articles").
        fetch_all(conn).await?.iter().map(|row| Article{
            id: Option::from(row.id as u32),
            title: row.title.clone(),
            content: row.content.clone(),
            create_date: Option::from(row.create_date),
        }).collect();
    Ok(Json(articles))
}

#[web::post("/article")]
async fn new_article
    (state: State<Arc<AppState>>, data: Json<Article>) -> Result<Json<Article>, CustomError> {
    let conn = &state.db_pool;

    let article = sqlx::query!("INSERT INTO articles (title, content, create_date) VALUES ($1, $2, $3) RETURNING id, title, content, create_date",
        data.title, data.content, chrono::Local::now().naive_local().date()).fetch_one(conn).await?;

    Ok(Json(Article{
        id: Option::from(article.id as u32),
        title: article.title,
        content: article.content,
        create_date: Option::from(article.create_date),
    }))
}

#[web::put("/article/{id}")]
async fn update_article
    (state: State<Arc<AppState>>, data: Json<Article>, id: Path<(u32,)>) -> Result<Json<Article>, CustomError> {
    let conn = &state.db_pool;
    let id = id.into_inner().0;
    let article = sqlx::query!("UPDATE articles SET title=$1, content=$2 WHERE id=$3 RETURNING id, title, content, create_date",
        data.title, data.content, id as i32).fetch_one(conn).await?; // Convert to i32 (optional)
    Ok(Json(Article{
        id: Option::from(article.id as u32),
        title: article.title,
        content: article.content,
        create_date: Option::from(article.create_date),
    }))
}

#[web::delete("/article/{id}")]
async fn delete_article
    (state: State<Arc<AppState>>, id: Path<(u32,)>) -> Result<Json<Article>, CustomError> {
    let conn = &state.db_pool;
    let id = id.into_inner().0;
    let article = sqlx::query!("DELETE FROM articles WHERE id=$1 RETURNING id, title, content, create_date",
        id as i32).fetch_one(conn).await?; // Convert to i32 (optional)
    Ok(Json(Article{
        id: Option::from(article.id as u32),
        title: article.title,
        content: article.content,
        create_date: Option::from(article.create_date),
    }))
}

#[web::get("/article/search/{keyword}")]
async fn search_article
    (state: State<Arc<AppState>>, keyword: Path<(String,)>) -> Result<Json<Vec<Article>>, CustomError> {
    let conn = &state.db_pool;
    let keyword = keyword.into_inner().0;
    let articles:Vec<Article> = sqlx::query!("SELECT * FROM articles WHERE title LIKE $1",
        format!("%{}%", keyword)).fetch_all(conn).await?.iter().map(|row| Article{
            id: Option::from(row.id as u32),
            title: row.title.clone(),
            content: row.content.clone(),
            create_date: Option::from(row.create_date),
        }).collect();
    Ok(Json(articles))
}

#[web::get("/article/{id}")]
async fn get_article
    (state: State<Arc<AppState>>, id: Path<(u32,)>) -> Result<Json<Article>, CustomError> {
    let conn = &state.db_pool;
    let id = id.into_inner().0;
    let article = sqlx::query!("SELECT * FROM articles WHERE id=$1",
        id as i32).fetch_one(conn).await?; // Convert to i32 (optional)
    Ok(Json(Article{
        id: Option::from(article.id as u32),
        title: article.title,
        content: article.content,
        create_date: Option::from(article.create_date),
    }))
}