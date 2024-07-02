use std::{str::FromStr, time::SystemTime};

use actix_web::web::Data;
use chrono::Utc;
use futures_util::StreamExt;
use mongodb::{bson::{doc, from_document, oid::ObjectId, DateTime}, error::Error, results::{InsertOneResult, UpdateResult}};

use crate::{models::{bookmod::booking::{Booking, Bookings}, dog::Dog, owner::Owner}, services::db::Database};

pub async fn create_owner(database: Data<Database>, owner: Owner) -> Result<InsertOneResult, Error> {
    let result = database
        .owner
        .insert_one(owner)
        .await
        .ok()
        .expect("Error Creating Owner");

    Ok(result)
}

pub async fn create_dog(database: Data<Database>, dog: Dog) -> Result<InsertOneResult, Error> {
    let result = database.dog.insert_one(dog).await.ok().expect("Could not create Dog");

    Ok(result)
}

pub async fn create_booking(database: Data<Database>, booking: Booking) -> Result<InsertOneResult, Error> {
    let result = database.booking.insert_one(booking).await.ok().expect("Could not creat booking");

    Ok(result)
}

pub async fn cancel_booking(database: Data<Database>, id: &str) -> Result<UpdateResult, Error> {
    let result = database
        .booking
        .update_one(
            doc! {
                "_id": ObjectId::from_str(id)
                    .expect("Invalid Id")
                }, 
                doc! {
                    "$set": doc! {
                        "cancelled": true
                    }
                })
                .await
                .ok()
                .expect("Could not update booking");

    Ok(result)
}

pub async fn fetch_all_bookings(database: Data<Database>) -> Result<Vec<Bookings>, Error> {

    //converting time
    let current_date_time: SystemTime = Utc::now().into();

    let mut results = database
        .booking
        .aggregate(vec![
            doc! {
                "$match": {
                    "cancelled": false,
                    "start_time": {
                        "$gte": DateTime::from_system_time(current_date_time)
                    }
                }
            },
            doc! {
                "$lookup": doc! {
                    "from": "owner",
                    "localField": "owner",
                    "foreignField": "_id",
                    "as": "owner"
                }
            },
            doc! {
                "$unwind": doc! {
                    "path": "$owner"
                }
            },
            doc! {
                "$lookup": doc! {
                    "from": "dog",
                    "localField": "owner._id",
                    "foreignField": "owner",
                    "as": "dog"
                }
            },
            doc! {
                "$unwind": doc! {
                    "path": "$owner"
                }
            },
        ])
        .await
        .ok()
        .expect("No Bookings");

    let mut bookings: Vec<Bookings> = Vec::new();

    //next is from the futures library
    //looping and format results into usable type
    while let Some(result) = results.next().await {
        match result {
            Ok(doc) => {
                let booking: Bookings = from_document(doc).expect("Could not format booking");
                bookings.push(booking);
            },
            Err(err) => panic!("Error formatting booking: {}", err)
        }
    }

    Ok(bookings)
}