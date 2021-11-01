extern crate abc;
extern crate diesel;

use abc::models::Experiment;
use rand::Rng;
use actix_web::{App, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use actix_web::web;
use std::env;

async fn get_experiment_session(req: HttpRequest, pool: web::Data<abc::DbPool>) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    };

    let experiment;
    let experiment_id = req.match_info().query("experiment_id");
    if let Some(e) = abc::get_experiment(&conn, experiment_id.to_string()) {
        experiment = e;
    } else {
        return HttpResponse::NotFound().finish()
    }

    if let Some(session_id) = req.cookie("AB-Session").map(|c| String::from(c.value())) {
        if let Some(session) = abc::get_session(&conn, &session_id, &experiment.id) {
            return HttpResponse::from(session);
        }
    }

    let mut rng = rand::thread_rng();
    return HttpResponse::from(abc::create_session(&conn, &experiment.id, rng.gen_range(0..experiment.variants)));
}

async fn post_experiment(body: web::Json<Experiment>, pool: web::Data<abc::DbPool>) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    };

    abc::create_experiment(&conn, &body);

    return HttpResponse::from(body.into_inner());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_host = env::var("BIND_HOST").expect("BIND_HOST must be set");
    let bind_port: u16 = env::var("BIND_PORT").expect("BIND_PORT must be set").parse().expect("Failed to parse BIND_PORT as u16");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Starting web server at {}:{} connecting to {}", bind_host, bind_port, database_url);
    
    let pool = abc::create_db_pool(database_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/experiment/{experiment_id}/session", web::get().to(get_experiment_session))
            .route("/experiment", web::post().to(post_experiment))
    })
    .bind((bind_host, bind_port))?
    .run()
    .await
}
