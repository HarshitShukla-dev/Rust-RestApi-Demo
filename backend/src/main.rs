mod config;
mod database;
mod routes;

use actix_web::{ web::Data, App, HttpServer };
use routes::user_route::create_user;
use config::mongodb_config::MongoConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoConfig::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}