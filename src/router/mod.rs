mod controller;
use actix_web::web;

pub fn implement_routes(app: &mut web::ServiceConfig) {
  app.route("/",web::get().to(controller::create_user));
}