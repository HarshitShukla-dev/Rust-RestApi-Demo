use crate::{ config::mongodb_config::MongoConfig, database::user_schema::User };
use actix_web::{ 
        post, 
        web::{ 
            Data, 
            Json 
        }, 
        HttpResponse 
};


#[post("/user")]
pub async fn create_user(db: Data<MongoConfig>, new_user: Json<User>) -> HttpResponse {
    let data = User{
        id: None,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        address: new_user.address.to_owned(),
        phone: new_user.phone.to_owned()
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(insert_result) => HttpResponse::Ok().json(insert_result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}