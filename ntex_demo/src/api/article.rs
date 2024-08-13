use std::sync::Arc;
use ntex::web::{
    self,
    types::{Json, State},
};

use crate::{errors::CustomError, models::article::Article, AppState};

#[web::get("/article")]
async fn get_articles(state: State<Arc<AppState>>) -> Result<Json<Vec<Article>>, CustomError> {
   let conn = &state.db_pool;
    let articles:Vec<Article> = sqlx::query!("SELECT * FROM articles").
        fetch_all(conn).await?.iter().map(|row| Article{
            id: row.id,
            title: row.title.clone(),
            content: row.content.clone(),
            create_date: row.create_date,
        }).collect();
    Ok(Json(articles))
}
