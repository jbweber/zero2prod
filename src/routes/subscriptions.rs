use actix_web::web::Form;
use actix_web::HttpResponse;

pub async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
