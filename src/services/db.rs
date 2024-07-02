use std::env;

use mongodb::{Client, Collection};

use crate::models::{bookmod::booking::Booking, dog::Dog, owner::Owner};

//struct for db
pub struct Database {
    pub booking: Collection<Booking>,
    pub dog: Collection<Dog>,
    pub owner: Collection<Owner>
}

//implementing database and connecting
impl Database {
    //init function
    pub async fn init() -> Self {
        let db_url = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://wedi:wedime@0.0.0.0:27017/dog_business?directConnection=true".to_string()
        };

        //creating a db client
        let client = Client::with_uri_str(db_url).await.unwrap();
        let db = client.database("dog_business");

        let booking: Collection<Booking> = db.collection("bookings");
        let dog: Collection<Dog> = db.collection("dogs");
        let owner: Collection<Owner> = db.collection("owners");

        Database {
            booking,
            dog,
            owner
        }
    }
}