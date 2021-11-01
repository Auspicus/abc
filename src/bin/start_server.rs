extern crate ab;
extern crate diesel;

use ab::models::Experiment;
use rand::Rng;
use actix_web::{App, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use actix_web::web;

async fn get_experiment_session(req: HttpRequest, pool: web::Data<ab::DbPool>) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    };

    let experiment;
    let experiment_id = req.match_info().query("experiment_id");
    if let Some(e) = ab::get_experiment(&conn, experiment_id.to_string()) {
        experiment = e;
    } else {
        return HttpResponse::NotFound().finish()
    }

    if let Some(session_id) = req.cookie("AB-Session").map(|c| String::from(c.value())) {
        if let Some(session) = ab::get_session(&conn, &session_id, &experiment.id) {
            return HttpResponse::from(session);
        }
    }

    let mut rng = rand::thread_rng();
    return HttpResponse::from(ab::create_session(&conn, &experiment.id, rng.gen_range(0..experiment.variants)));
}

async fn post_experiment(body: web::Json<Experiment>, pool: web::Data<ab::DbPool>) -> HttpResponse {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    };

    ab::create_experiment(&conn, &body);

    return HttpResponse::from(body.into_inner());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = ab::create_db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/experiment/{experiment_id}/session", web::get().to(get_experiment_session))
            .route("/experiment", web::post().to(post_experiment))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
