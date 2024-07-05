use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::database::user_schema::User;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    Client, Collection,
};

pub struct MongoConfig {
    col: Collection<User>,
}

impl MongoConfig {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading environment variable."),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDb");
        let col: Collection<User> = db.collection("User");
        MongoConfig { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            username: new_user.username,
            email: new_user.email,
            address: new_user.address,
            phone: new_user.phone,
        };
        let insert_result = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating the user");
        Ok(insert_result)
    }

    pub async fn fetch_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's details.");
        Ok(user_detail.unwrap())
    }
}
