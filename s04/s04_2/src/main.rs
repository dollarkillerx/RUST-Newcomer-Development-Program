#[macro_use]
extern crate lazy_static;

use async_std::prelude::*;
use std::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use serde::{Serialize, Deserialize};

use md5;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[async_std::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    init_db().await?;
    // create_db().await?;
    fund().await?;
    Ok(())
}


lazy_static! {
  // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，内部使用DashMap等等并发安全的map实现，无需担心线程竞争
  static ref RB:Rbatis=Rbatis::new();
}

async fn init_db() -> Result<()> {
    //启用日志输出，你也可以使用其他日志框架，这个不限定的
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
    //初始化连接池
    RB.link("mysql://root:root@127.0.0.1:3306/test").await?;
    Ok(())
}

async fn create_db() -> Result<()> {
    let create = r#"
    CREATE TABLE `users` (
	`id` BIGINT NOT NULL AUTO_INCREMENT,
	`account` VARCHAR(255) NOT NULL COMMENT "account",
	`password` CHAR(32) NOT NULL COMMENT "password",
	PRIMARY KEY(`id`),
	UNIQUE KEY`account`(`account`)
    )ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8;
    "#;

    RB.exec("", create).await?;

    // let p = RB.exec("","show tables;").await?;
    // println!("{:?}",p);

    Ok(())
}

#[macro_use]
extern crate rbatis_macro_driver;

#[crud_enable(id_name: id | table_name: users | table_columns: id, account, password)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    pub id: Option<u64>,
    pub account: Option<String>,
    pub password: Option<String>,
}

async fn fund() -> Result<()> {
    let u1 = User {
        id: None,
        account: Some("dollarkiller".to_string()),
        password: Some(format!("{:x}", md5::compute("password"))),
    };
    println!("{:#?}",u1);
    RB.save("", &u1).await?;

    let q1 = RB.new_wrapper().eq("account", "dollarkiller").
        check().unwrap();

    let r: std::result::Result<Option<User>, rbatis::core::Error> = RB.fetch_by_wrapper("", &q1).await;
    println!("{:?}", r);
    // let w = rb.new_wrapper().eq("id", "1").check().unwrap();
    // let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper("", &w).await;
    Ok(())
}