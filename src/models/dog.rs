use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

//creating a public data struct
//model for mongo db
#[derive(Debug, Serialize, Deserialize)]
pub struct Dog {
    pub owner: ObjectId,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>
}

//for request
#[derive(Debug, Serialize, Deserialize)]
pub struct DogRequest {
    pub owner: String,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>
}

//implementing a trait to convert booking request to booking
impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: DogRequest) -> Result<Self, Self::Error> {
        
        Ok(Self {
            owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner"),
            name: item.name,
            age: item.age,
            breed: item.breed,
        })
    }
}