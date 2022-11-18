use actix_web::http::header::ContentType;
use actix_web::web::{Data, Form};
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs;

struct AppState {
    std_code: String,
}

#[derive(Deserialize)]
struct RunRequest {
    code: String,
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
            .app_data(get_app_data())
            .service(run_service)
            .service(actix_files::Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn get_app_data() -> Data<AppState> {
    Data::new(AppState {
        std_code: get_std_code(),
    })
}

fn get_std_code() -> String {
    fs::read_to_string("../compost/lib/std.compost").expect("Unable to locate std.compost")
}
