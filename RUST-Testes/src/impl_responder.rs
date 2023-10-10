use actix_web::{HttpResponse, Responder, get, web};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Este é um exemplo simples de como usar impl Responder em Rust com o framework Actix Web
//para criar uma rota básica de API da web que retorna uma resposta HTTP.
