mod db;
mod handlers;
mod routes;
mod schema;

use actix_web::{web, App, HttpServer};
use db::connection::establish_connection;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(routes::init())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
