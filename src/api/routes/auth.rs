use actix_web::{get, post, web, HttpResponse, HttpRequest, Result};

use crate::jwt::{encode, decode, Claims};

struct AuthCredentials {
  usename: String,
  password: String,
}


#[post("/login")]
pub async fn login(body: web::Data<AuthCredentials>) -> Result<HttpResponse> {

  Ok(HttpResponse::Ok().finish())
}

#[get("/api/create-jwt")]
pub async fn create_jwt() -> Result<HttpResponse> {
  let claims = Claims::new(
    "23ioxfk2e",
    "Jacob",
    "admin",
    &vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    15
  );
  let token = encode(&claims).unwrap();
  Ok(HttpResponse::Ok().body(token))
}


#[get("/api/decode-jwt/{token}")]
pub async fn decode_jwt(params: web::Path<(String,)>) -> Result<HttpResponse> {
  let (token,) = params.into_inner();
  match decode(&token) {
    Ok(data) => Ok(HttpResponse::Ok().json(data.claims)),
    Err(error) => {
      println!("{:?}", error);
      Ok(HttpResponse::BadRequest().json(json!({
        "error": "Uh ohhh, bad boy!"
      })))
    }
  }
  
}
