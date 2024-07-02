use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

//creating a public data struct
//model for mongo db
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String
}

//for request
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String
}

//implementing a trait to convert booking request to booking
impl TryFrom<OwnerRequest> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: OwnerRequest) -> Result<Self, Self::Error> {
        
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            email: item.email,
            phone: item.phone,
            address: item.address
        })
    }
}