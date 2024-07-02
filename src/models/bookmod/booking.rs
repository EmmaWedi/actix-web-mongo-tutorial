use std::time::SystemTime;

use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::models::{dog::Dog, owner::Owner};

//creating a public data struct
//model for mongo db
#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

//for request
#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
}

//creating bookings type
#[derive(Debug, Serialize, Deserialize)]
pub struct Bookings {
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

//implementing a trait to convert booking request to booking
impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: BookingRequest) -> Result<Self, Self::Error> {
        //converting date time using chrono
        let ch_datetime: SystemTime =  chrono::DateTime::parse_from_rfc3339(&item.start_time).map_err(|err| format!("Failed To Parse Time:, {}", err))?.with_timezone(&Utc).into();

        Ok(Self {
            owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner"),
            start_time: DateTime::from(ch_datetime),
            duration_in_minutes: item.duration_in_minutes,
            cancelled: false
        })
    }
}