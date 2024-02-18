use actix_web::web;
use serde::Serialize;
#[derive(Serialize)]
pub struct Response {
  name: String,
  age: i16,
}

pub async fn create_user() -> web::Json<Response> {
  let user = Response{
    age: 20,
    name: "João Pedro".to_owned()
  };
  web::Json(user)
}