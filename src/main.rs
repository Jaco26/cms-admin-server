#![allow(non_snake_case)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use std::{env, io};
use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod database;
mod api;
mod jwt;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let PORT = env::var("PORT").unwrap_or(String::from("3000"));
    let IP = "0.0.0.0";

    let db = database::Db::new(&DATABASE_URL).expect("Couldn't create instance of `Db`");

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .service(api::heartbeat)
    })
    .bind(format!("{}:{}", IP, PORT))?
    .run()
    .await
}
