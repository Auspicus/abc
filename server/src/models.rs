use super::schema::experiments;
use super::schema::sessions;
use actix_web::http::Cookie;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "sessions"]
pub struct Session {
    pub id: String,
    pub experiment_id: String,
    pub variant: i32,
}

#[derive(Insertable, Queryable, Clone, Deserialize, Serialize)]
#[table_name = "experiments"]
pub struct Experiment {
    pub id: String,
    pub variants: i32,
}

impl From<Session> for HttpResponse {
    fn from(s: Session) -> Self {
        let body = serde_json::to_string(&s).expect("Failed to serialize session.");

        HttpResponse::Ok()
            .cookie(
                Cookie::build("AB-Session", s.id.to_string())
                    .permanent()
                    .finish(),
            )
            .cookie(Cookie::new("AB-Variant", format!("{}", s.variant)))
            .header("Cache-Control", "no-store, max-age=0")
            .body(body)
    }
}

impl From<Experiment> for HttpResponse {
    fn from(e: Experiment) -> Self {
        HttpResponse::Ok().json(e)
    }
}
