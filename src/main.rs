mod db;
mod handlers;
mod routes;
mod schema;

use actix_web::{App, HttpServer};
use db::connection::establish_connection;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .app_data(establish_connection().clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
