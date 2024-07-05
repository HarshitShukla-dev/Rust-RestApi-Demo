use crate::{config::mongodb_config::MongoConfig, database::user_schema::User};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/user")]
pub async fn create_user(db: Data<MongoConfig>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        address: new_user.address.to_owned(),
        phone: new_user.phone.to_owned(),
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(insert_result) => HttpResponse::Ok().json(insert_result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn fetch_user(db: Data<MongoConfig>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid id.");
    }
    let user_detail = db.fetch_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
