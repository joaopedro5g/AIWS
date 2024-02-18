mod router;

use actix_web::{ App, HttpServer };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().configure(router::implement_routes)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}