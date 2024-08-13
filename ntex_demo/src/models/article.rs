use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub create_date: chrono::NaiveDate,
}