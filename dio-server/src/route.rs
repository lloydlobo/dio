//! `actors` acts as the routes controller for REST Api.
//!
//! TODO: Add [`services`]() to implementDB logic like `find_one` ...
//!
//! See https://github.com/actix/examples/blob/master/databases/mongodb/src/main.rs

use crate::model::{Facts, Principles};
use actix_web::{get, post, web, HttpResponse, Responder};
use dio_server::{COLL_NAME_FACTS, COLL_NAME_PRINCIPLES, DB_NAME};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};

// -> HttpResponse | impl Responder
#[get("/facts/{id}")]
async fn get_fact(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let collection: Collection<Facts> = client.database(DB_NAME).collection(COLL_NAME_FACTS);

    let find_one = collection.find_one(doc! {"id": &id}, None).await;

    match find_one {
        Ok(Some(fact)) => HttpResponse::Ok().json(fact), // Ok(None) => HttpResponse::NotFound().finish(),
        Ok(None) => HttpResponse::NotFound().body(format!("No fact found with id {id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/facts")]
async fn get_facts() -> impl Responder {
    HttpResponse::Ok().json("Ok")
    // let collection: Collection<Facts> = client.database(DB_NAME).collection(COLL_NAME_FACTS);
    // dbg!(&collection);
    // let find_all = collection.find(None, None).await;
    // match find_all {
    //     Ok(facts) => {
    //         let deserialize_current = facts.deserialize_current();
    //         let deserialize_current = deserialize_current.unwrap();
    //         HttpResponse::Ok().json(deserialize_current)
    //     }
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
}

#[post("/principles")]
async fn create_principle(
    client: web::Data<Client>, // form: web::Form<Principles>,
    param_obj: web::Json<Principles>,
    // TODO: Get body data here.
) -> impl Responder {
    let collection = client.database(DB_NAME).collection(COLL_NAME_PRINCIPLES); // let inner = form.into_inner();
    let inner = Principles {
        id: param_obj.id.clone(),
        title: param_obj.title.clone(),
    };
    let result = collection.insert_one(inner, None).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().body(format!("Created principle with id: {}", res.inserted_id))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/principles/{id}")]
async fn get_principle(client: web::Data<Client>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let collection: Collection<Facts> = client.database(DB_NAME).collection(COLL_NAME_PRINCIPLES);
    match collection.find_one(doc! {"id": &id}, None).await {
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
    let collection: Collection<Principles> =
        client.database(DB_NAME).collection(COLL_NAME_PRINCIPLES);

    let find = collection.find(None, None).await;

    match find {
        Ok(principles) => {
            let deserialize_current = principles.deserialize_current();
            let deserialize_current = deserialize_current.unwrap();
            dbg!(&deserialize_current);
            HttpResponse::Ok().json(deserialize_current)
        }
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
        .service(echo) // TODO: Remove
        .route("/hey", web::get().to(manual_hello)); // TODO: Remove
}

// client
//     .database("dio")
//     .collection("facts")
//     .find_one(doc! {"_id": id.into_inner()}, None),

// TODO: Route index to repository.
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World! Go to the GitHub repository.")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json("Ok")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
