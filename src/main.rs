use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::{
    booking::{cancel_booking, create_booking, get_all_bookings},
    dog::create_dog,
    owner::create_owner,
};
use services::db::Database;

//mods
mod models;
mod routes;
mod services;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server Running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize db
    let db_conn = Database::init().await;
    //creating a data object
    let db_data = Data::new(db_conn);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_booking)
            .service(create_dog)
            .service(create_owner)
            .service(cancel_booking)
            .service(get_all_bookings)
    })
    .bind(("0.0.0.0", 6000))?
    .run()
    .await
}
