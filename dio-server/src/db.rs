//! See [Reference source code](https://github.com/Mr-Malomz/actix-mongo-api/blob/main/src/repository/mongodb_repo.rs).
use super::model::{Facts, Principles};
use crate::util::get_env_var;
use anyhow::Context;
use dotenv::dotenv;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};

// use futures::stream::TryStreamExt;
// See https://github.com/actix/examples/tree/master/databases/mongodb
#[allow(unused)]
pub struct DioDB {
    coll_facts: Collection<Facts>,
    coll_principles: Collection<Principles>,
}

impl DioDB {
    pub async fn init() -> Client {
        dotenv().ok();

        // Load the MongoDB connection string from an environment variable:
        let client_uri = get_env_var("MONGODB_URI")
            .context(
                "MONGODB_URI environment variable not set. \
                Please set the MONGODB_URI environment variable to the URI of your MongoDB instance \
                (e.g. mongodb://localhost:27017).",
            )
            .expect("Should get environment variable for database URI.");
        // dbg!(&client_uri);

        // Workaround for a DNS issue on Windows:
        let options: ClientOptions =
            match ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await {
                Ok(t) => t,
                Err(e) => unwrap_failed_options("called `Result::unwrap()` on an `Err` value", &e),
            };

        // A Client is needed to connect to MongoDB:
        let client: Client = match Client::with_options(options) {
            Ok(t) => t,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
        };
        // dbg!(&client);
        client

        // let db: mongodb::Database = client.database("dio");
        // println!("Connected to the database: {}", db.name().to_string());

        // let coll_facts: Collection<Facts> = db.collection::<Facts>("facts");
        // let coll_principles: Collection<Principles> = db.collection::<Principles>("principles");

        // DioDB {
        //     coll_facts,
        //     coll_principles,
        // }
    }
}

fn unwrap_failed_options(arg: &str, e: &mongodb::error::Error) -> ClientOptions {
    eprintln!(
        "Failed to connect while parsing the MongoDB URI connection string: {}.\nThe error was: {}",
        arg, e
    );
    std::process::exit(1);
}

fn unwrap_failed(arg: &str, e: &mongodb::error::Error) -> Client {
    eprintln!("Failed to connect to MongoDB: {arg}. Error: {e}");
    std::process::exit(1);
}

/// .
///
/// # Panics
///
/// Panics if .
#[allow(unused)]
pub async fn print_db_coll_names(client: Client, db: &mongodb::Database) {
    // Print the databases in our MongoDB cluster:
    println!("Databases");
    for name in client
        .list_database_names(None, None)
        .await
        .context("Failed to list databases in the MongoDB cluster.")
        .expect("Should get database names from the client.")
    {
        println!("- {name}");
    }

    // Print the collection in our "dio" MongoDB database in cluster:
    println!("Collections");
    for collection_name in db
        .list_collection_names(None)
        .await
        .context("Failed to list collections in the MongoDB database.")
        .expect("Should get collection names from the database.")
    {
        println!("- {collection_name}");
    }
}

// let new_doc = doc! { "title": "Parasite", "year": 2020, "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.", }; // "released": Utc.with_ymd_and_hms(2020, 2, 7, 0, 0, 0),
// let insert_facts_result = coll_facts.insert_one(new_doc.clone(), None).await.unwrap();
// println!(
// "Inserted with _id: {:?} in collection: {}", insert_facts_result.inserted_id, coll_facts.name(),
// );
// let insert_principle_result = coll_principles .insert_one(new_doc.clone(), None) .await .unwrap();
// println!(
// "Inserted with _id: {:?} in collection: {}", insert_principle_result.inserted_id, coll_principles.name(),
// );
