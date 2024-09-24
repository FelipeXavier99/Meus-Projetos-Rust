use actix_web::{web, App, HttpServer, HttpResponse};
use minijinja::{Environment, Source};

async fn index(env: web::Data<Environment<'static>>) -> HttpResponse {
    let tmpl = env.get_template("index.html").unwrap();
    let rendered = tmpl.render(&()).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurando Minijinja
    let mut env = Environment::new();
    let mut source = Source::new();
    source.add_template("index.html", "<h1>Hello, Minijinja com Actix!</h1>").unwrap();
    env.set_source(source);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(env.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
