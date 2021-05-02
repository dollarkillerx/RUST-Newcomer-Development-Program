use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

#[get("/index.html")]
async fn index1() -> impl Responder {
    format!("Hello World")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index1))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}