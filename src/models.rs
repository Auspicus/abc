use super::schema::sessions;
use actix_web::HttpResponse;
use actix_web::http::{Cookie};

#[derive(Insertable, Queryable)]
#[table_name = "sessions"]
pub struct Session {
    pub id: String,
    pub variant: i32,
}

impl From<Session> for HttpResponse {
    fn from(s: Session) -> Self {
        HttpResponse::Ok()
            .cookie(Cookie::build("AB-Session", format!("{}", s.id)).permanent().finish())
            .cookie(Cookie::new("AB-Variant", format!("{}", s.variant)))
            .finish()
    }
}