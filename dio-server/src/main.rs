//! dio-server is the backend for REST Api integration with MongoDB.
//!
//! # TODO:

extern crate dotenv;

use crate::{db::DioDB, route::config};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

mod db;
pub mod model;
mod route;
mod util;
// #[cfg(test)]
// mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_client: Client = DioDB::init().await;
    const PORT: u16 = 5000;
    println!("Starting server on PORT {}", PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
