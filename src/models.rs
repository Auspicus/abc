use super::schema::sessions;
use super::schema::experiments;
use actix_web::{HttpResponse};
use actix_web::http::{Cookie};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable)]
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
        HttpResponse::Ok()
            .cookie(Cookie::build("AB-Session", format!("{}", s.id)).permanent().finish())
            .cookie(Cookie::new("AB-Variant", format!("{}", s.variant)))
            .finish()
    }
}

impl From<Experiment> for HttpResponse {
    fn from(e: Experiment) -> Self {
        HttpResponse::Ok().json(e)
    }
}