use actix_web::{get, http::Error, post, put, web::{Data, Json, Path}, HttpResponse};

use crate::{models::bookmod::booking::{Booking, BookingRequest}, services::{db::Database, testing}};

#[post("/booking")]
pub async fn create_booking(database: Data<Database>, req: Json<BookingRequest>) -> Result<HttpResponse, Error> {
    match testing::create_booking(database, Booking::try_from(BookingRequest {
        owner: req.owner.clone(),
        start_time: req.start_time.clone(),
        duration_in_minutes: req.duration_in_minutes.clone(),
    })
    .expect("Could Not Parse Booking")
    ).await {
        Ok(booking) => Ok(HttpResponse::Ok().json(booking)),
        Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string()))
    }
}

#[get("/all")]
pub async fn get_all_bookings(database: Data<Database>) -> Result<HttpResponse, Error> {
    match testing::fetch_all_bookings(database).await {
        Ok(bookings) => Ok(HttpResponse::Ok().json(bookings)),
        Err(err) => Ok(HttpResponse::BadRequest().body(err.to_string()))
    }
}

#[put("/cancel/{id}")]
pub async fn cancel_booking(database: Data<Database>, path: Path<(String, )>) -> Result<HttpResponse, Error> {
    let id = path.into_inner().0;

    match testing::cancel_booking(database, &id).await {
        Ok(booking) => Ok(HttpResponse::Ok().json(booking)),
        Err(err) =>  Ok(HttpResponse::InternalServerError().body(err.to_string()))
    }
}