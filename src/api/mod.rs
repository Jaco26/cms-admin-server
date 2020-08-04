use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use crate::database::Db;

pub mod model;

use model::api_response::ApiResponse;

#[derive(Serialize)]
struct Bla {
  message: String
}

#[get("/api/heartbeat")]
pub async fn heartbeat(db: web::Data<Db>) -> HttpResponse {
  let mut res = ApiResponse::new();

  res.set_data(json!("thump thump"));

  HttpResponse::Ok().json(res)
}