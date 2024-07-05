mod config;
mod database;
mod routes;

use actix_web::{web::Data, App, HttpServer};
use config::mongodb_config::MongoConfig;
use routes::user_route::{create_user, fetch_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoConfig::init().await;
    let db_data = Data::new(db);
    let server_address = ("localhost", 8080);

    println!("Server running at http://{}:{}", server_address.0, server_address.1);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(fetch_user)
    })
    .bind(server_address)?
    .run()
    .await
}