use actix_web::{http::Error, post, web::{Data, Json}, HttpResponse};

use crate::{models::dog::{Dog, DogRequest}, services::{db::Database, testing}};

#[post("/dog")]
pub async fn create_dog(database: Data<Database>, req: Json<DogRequest>) -> Result<HttpResponse, Error> {
    match testing::create_dog(database, Dog::try_from(DogRequest {
        owner: req.owner.clone(),
        name: req.name.clone(),
        age: req.age.clone(),
        breed: req.breed.clone()
    })
    .expect("Error Parsing Request")
    ).await {
        Ok(dog) => Ok(HttpResponse::Ok().json(dog)),
        Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string()))
    }
}