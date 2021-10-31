extern crate ab;
extern crate diesel;

use rand::Rng;
use actix_web::{web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer};

async fn experiment_session_handler(req: HttpRequest, pool: web::Data<ab::DbPool>) -> HttpResponse {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = ab::create_db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/{experiment_id}", web::get().to(experiment_session_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
