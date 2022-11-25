use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::web::Form;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[get("/")]
async fn index_service() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("resources/index.html"))
}

#[post("/run")]
async fn run_service(form: Form<RunRequest>) -> impl Responder {
    let result = compost::run::run_code(&form.code);

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(get_cors_middleware())
            .service(index_service)
            .service(run_service)
    })
    .bind(("0.0.0.0", get_port()))?
    .run()
    .await
}

fn get_cors_middleware() -> Cors {
    Cors::default()
        .allowed_origin("http://compost-playground.sytzez.com")
        .allowed_origin(&format!("http://localhost:{}", get_port()))
        .allowed_methods(vec!["GET", "POST"])
        .max_age(3600)
}

fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid PORT env variable provided")
}
