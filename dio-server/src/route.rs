//! `actors` acts as the routes controller for REST Api.
//!
//! TODO: Add [`services`]() to implement DB logic like `find_one` ...
//!
//! See [`MongoDB` examples](https://github.com/actix/examples/blob/master/databases/mongodb/src/main.rs)

use crate::model::{Facts, Principles};
use actix_web::{get, post, web, HttpResponse, Responder};
use anyhow::Context;
use dio_server::{COLL_NAME_FACTS, COLL_NAME_PRINCIPLES, DB_NAME};
use mongodb::{bson::doc, Client, Collection};

// -> HttpResponse | impl Responder
#[get("/facts/{id}")]
async fn get_fact(client: web::Data<Client>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let collection: Collection<Facts> = client.database(DB_NAME).collection(COLL_NAME_FACTS);

    let result = collection
        .find_one(doc! {"id": &id}, None)
        .await
        .context(format!("Failed to find fact with id: {id}"));
    match result {
        Ok(Some(fact)) => HttpResponse::Ok().json(fact), // Ok(None) => HttpResponse::NotFound().finish(),
        Ok(None) => HttpResponse::NotFound().body(format!("No fact found with id {id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/facts")]
async fn get_facts(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Facts> = client.database(DB_NAME).collection(COLL_NAME_FACTS);

    let result = collection
        .find(None, None)
        .await
        .context("Error while finding facts in get_facts_new");
    match result {
        Ok(it) => {
            let facts = it
                .deserialize_current()
                .context("Error while deserializing facts in get_facts_new")
                .expect("Should Deserialize the current result to the generic type Facts associated with this cursor.");
            HttpResponse::Ok().json(facts)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/facts")]
async fn create_fact(client: web::Data<Client>, param_obj: web::Json<Facts>) -> impl Responder {
    let collection = client.database(DB_NAME).collection(COLL_NAME_FACTS);
    let inner = Facts {
        id: param_obj.id,
        title: param_obj.title.clone(),
    };

    let result = collection
        .insert_one(inner, None)
        .await
        .context("Error while inserting fact in create_fact");
    match result {
        Ok(res) => HttpResponse::Ok().body(format!("Create fact with id: {}", res.inserted_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/principles/{id}")]
async fn get_principle(client: web::Data<Client>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let collection: Collection<Facts> = client.database(DB_NAME).collection(COLL_NAME_PRINCIPLES);

    let result = collection
        .find_one(doc! {"id": &id}, None)
        .await
        .context(format!("Failed to find principle with id: {id}"));
    match result {
        Ok(Some(fact)) => HttpResponse::Ok().json(fact), // Ok(None) => HttpResponse::NotFound().finish(),
        Ok(None) => HttpResponse::NotFound().body(format!("No principle found with id {id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Results from queries are generally returned via Cursor, a struct which
/// streams the results back from the server as requested. The Cursor type
/// implements the Stream trait from the futures crate, and in order to access
/// its streaming functionality you need to import at least one of the StreamExt
/// or TryStreamExt traits.
#[get("/principles")]
async fn get_principles(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Principles> = client.database(DB_NAME).collection(COLL_NAME_PRINCIPLES);

    let result = collection
        .find(None, None)
        .await
        .context("Error while finding principles in get_principles");
    match result {
        Ok(it) => {
            let principles = it
                .deserialize_current()
                .context("Error deserializing current principle")
                .expect("Error deserializing current principle");
            HttpResponse::Ok().json(principles)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/principles")]
async fn create_principle(client: web::Data<Client>, param_obj: web::Json<Principles>) -> impl Responder {
    let collection = client.database(DB_NAME).collection(COLL_NAME_PRINCIPLES); // let inner = form.into_inner();
    let inner = Principles {
        id: param_obj.id,
        title: param_obj.title.clone(),
    };

    let result = collection
        .insert_one(inner, None)
        .await
        .context("Error while inserting principle in create_principle");
    match result {
        Ok(res) => HttpResponse::Ok().body(format!("Created principle with id: {}", res.inserted_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(healthcheck)
        .service(get_fact)
        .service(get_facts)
        .service(get_principle)
        .service(get_principles)
        .service(create_principle)
        .service(create_fact)
        .service(echo) // TODO: Remove this service
        .route("/hey", web::get().to(manual_hello)); // TODO: Remove this route
}

// client
//     .database("dio")
//     .collection("facts")
//     .find_one(doc! {"_id": id.into_inner()}, None),

// TODO: Route index to repository.
#[allow(clippy::unused_async)]
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World! Go to the GitHub repository.")
}

#[allow(clippy::unused_async)]
#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json("Ok")
}

#[allow(clippy::unused_async)]
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[allow(clippy::unused_async)]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
