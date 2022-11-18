use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::web::{Data, Form};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs;

struct AppState {
    std_code: String,
    index_html: String,
}

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[get("/")]
async fn index_service(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(state.index_html.clone())
}

#[post("/run")]
async fn run_service(form: Form<RunRequest>, state: Data<AppState>) -> impl Responder {
    let result = compost::run::run_code(&(state.std_code.clone() + &form.code));

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(get_cors())
            .app_data(get_app_data())
            .service(index_service)
            .service(run_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn get_app_data() -> Data<AppState> {
    Data::new(AppState {
        std_code: get_std_code(),
        index_html: get_index_html(),
    })
}

fn get_std_code() -> String {
    fs::read_to_string("../compost/lib/std.compost").expect("Unable to locate std.compost")
}

fn get_index_html() -> String {
    fs::read_to_string("./public/index.html").expect("Unable to locate index.html")
}

fn get_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://compost-playground.sytzez.com")
        .allowed_origin("http://localhost:8080")
        .allowed_methods(vec!["GET", "POST"])
        .max_age(3600)
}
