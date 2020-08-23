use actix_web::{get, web, HttpResponse, Result};

use crate::jwt::{encode, decode, Claims};


#[get("/api/create-jwt")]
pub async fn create_jwt() -> Result<HttpResponse> {
  let claims = Claims::new(
    "23ioxfk2e",
    "Jacob",
    "admin",
    &vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    60 * 15
  );
  let token = encode(&claims).unwrap();
  Ok(HttpResponse::Ok().body(token))
}


#[get("/api/decode-jwt/{token}")]
pub async fn decode_jwt(params: web::Path<(String,)>) -> Result<HttpResponse> {
  let (token,) = params.into_inner();
  let data = decode(&token).unwrap();
  Ok(HttpResponse::Ok().json(data.claims))
}
