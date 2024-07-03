use std::env;
extern crate dotenv;
use actix_web::{dev::Url, http::uri};
use dotenv::dotenv;

use mongodb::{
    bson::{ extjson::de::Error },
    results::{ InsertOneResult },
    Client, Collection,
};
use crate::database::user_schema::User;

pub struct MongoConfig {
    col: Collection<User>,
}

impl MongoConfig {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURL"){
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading environment variable.")
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDb");
        let col: Collection<User> = db.collection("User");
        MongoConfig { col }
    }
}