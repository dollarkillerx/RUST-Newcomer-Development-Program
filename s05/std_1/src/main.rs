use async_std::prelude::*;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

use mongodb::{Client, options::ClientOptions};

#[async_std::main]
async fn main() -> Result<()> {
    mongo_1().await?;
    mongo_2().await?;
    mongo_3().await?;
    mongo_4().await?;
    Ok(())
}


async fn mongo_1() -> Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://root:root@127.0.0.1:27017").await?;

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options)?;

    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
}

async fn mongo_2() -> Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://root:root@127.0.0.1:27017").await?;

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options)?;

    let db = client.database("mydb");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}

use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

async fn mongo_3() -> Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://root:root@127.0.0.1:27017").await?;

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options)?;

    let db = client.database("mydb");
    let collection = db.collection("book");
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    collection.insert_many(docs, None).await?;

    Ok(())
}

async fn mongo_4() -> Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://root:root@127.0.0.1:27017").await?;

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options)?;

    let db = client.database("mydb");
    let collection = db.collection("book");

    let filter = doc! { "author": "George Orwell" };
    // let find_options = FindOptions::builder().sort(doc! {"title": 1}).build();
    // let mut cursor = collection.find(filter, find_options).await?;

    let mut cursor = collection.find(filter, None).await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    println!("title: {}", title);
                } else {
                    println!("no title found");
                }
            }
            Err(e) => return Err(e.into())
        }
    }

    Ok(())
}