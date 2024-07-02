use actix_web::{http::Error, post, web::{Data, Json}, HttpResponse};

use crate::{models::owner::{Owner, OwnerRequest}, services::{db::Database, testing}};

#[post("/owner")]
pub async fn create_owner(database: Data<Database>, req: Json<OwnerRequest>) -> Result<HttpResponse, Error> {
    match testing::create_owner(database, Owner::try_from(OwnerRequest {
        email: req.email.clone(),
        name: req.name.clone(),
        phone: req.phone.clone(),
        address: req.address.clone()
    })
    .expect("Error Parsing Owner")
    ).await {
        Ok(owner) => Ok(HttpResponse::Ok().json(owner)),
        Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string()))
    }
}