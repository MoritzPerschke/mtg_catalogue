mod db;
mod handlers;
mod routes;
mod schema;

use actix_web::{App, HttpServer, web, HttpResponse};
use db::connection::establish_connection;

use handlers::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .app_data(establish_connection().clone())
            .service(web::scope("/card").configure(card::config))
            .route("/", web::to(|| async {HttpResponse::Ok().body("Hello from mtg catalogue api")}))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
