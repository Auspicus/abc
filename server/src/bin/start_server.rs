extern crate abc;
extern crate diesel;

use abc::models::{Experiment, Session};
use actix_cors::Cors;
use actix_web::{http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use rand::Rng;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct SessionResultPage {
    data: Vec<Session>,
}

async fn get_sessions(_: HttpRequest, pool: web::Data<abc::DbPool>) -> HttpResponse {
    let conn = pool
        .get()
        .expect("Failed to get connection from database pool.");

    let body = serde_json::to_string(&SessionResultPage {
        data: abc::get_sessions(&conn, 5),
    })
    .expect("Failed to serialize response.");

    HttpResponse::Ok().body(body)
}

async fn get_experiment_session(req: HttpRequest, pool: web::Data<abc::DbPool>) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let experiment;
    let experiment_id = req.match_info().query("experiment_id");
    if let Some(e) = abc::get_experiment(&conn, experiment_id.to_string()) {
        experiment = e;
    } else {
        return HttpResponse::NotFound().finish();
    }

    if let Some(session_id) = req.cookie("AB-Session").map(|c| String::from(c.value())) {
        if let Some(session) = abc::get_session(&conn, &session_id, &experiment.id) {
            return HttpResponse::from(session);
        }
    }

    let mut rng = rand::thread_rng();
    HttpResponse::from(abc::create_session(
        &conn,
        &experiment.id,
        rng.gen_range(0..experiment.variants),
    ))
}

async fn post_experiment(
    body: web::Json<Experiment>,
    pool: web::Data<abc::DbPool>,
) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    abc::create_experiment(&conn, &body);

    HttpResponse::from(body.into_inner())
}

#[derive(Serialize)]
struct ExperimentResultPage {
    data: Vec<Experiment>,
}

async fn get_experiments(pool: web::Data<abc::DbPool>) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let body = serde_json::to_string(&ExperimentResultPage {
        data: abc::get_experiments(&conn, 5),
    })
    .expect("Failed to serialize response.");

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_host = env::var("BIND_HOST").expect("BIND_HOST must be set");
    let bind_port: u16 = env::var("BIND_PORT")
        .expect("BIND_PORT must be set")
        .parse()
        .expect("Failed to parse BIND_PORT as u16");
    let database_url = env::var("RUNTIME_DATABASE_URL").expect("RUNTIME_DATABASE_URL must be set");
    println!(
        "Starting web server at {}:{} connecting to {}",
        bind_host, bind_port, database_url
    );

    let pool = abc::create_db_pool(database_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(pool.clone())
            .route(
                "/experiments/{experiment_id}/session",
                web::get().to(get_experiment_session),
            )
            .route("/experiments", web::post().to(post_experiment))
            .route("/experiments", web::get().to(get_experiments))
            .route("/sessions", web::get().to(get_sessions))
    })
    .bind((bind_host, bind_port))?
    .run()
    .await
}
