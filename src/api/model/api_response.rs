use actix_web::Responder;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse {
  errors: Option<Vec<String>>,
  data: Option<Value>,
}

impl ApiResponse {
  pub fn new() -> ApiResponse {
    ApiResponse {
      errors: None,
      data: None,
    }
  }
  pub fn set_data(&mut self, data: Value) {
    self.data = Some(data);
  }
  pub fn add_error(&mut self, error: &str) {
    if self.errors.is_none() {
      self.errors = Some(vec![error.to_string()]);
    } else {
      self.errors.as_mut().unwrap().push(error.to_string());
    }
  }
}

