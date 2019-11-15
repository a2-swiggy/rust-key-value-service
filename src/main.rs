#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod schema;
mod routes;

use self::diesel::prelude::*;
use crate::models::Pair;
use schema::config::dsl::config;
use actix_web::{web, HttpServer, App};

fn main() {
    let connection = db::establish_connection();
    let results = config.load::<Pair>(&connection)
        .expect("Error loading key value pairs");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::greet))
            .route("/", web::post().to(routes::greet))
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
