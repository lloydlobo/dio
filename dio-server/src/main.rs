//! dio-server is the backend for REST Api integration with `MongoDB`.
//!
//! # TODO:

#![forbid(unsafe_code, non_ascii_idents)]
#![warn(clippy::pedantic)]
#![deny(
    dead_code, missing_docs, noop_method_call, pointer_structural_match, rust_2018_idioms, rust_2021_compatibility, trivial_casts, trivial_numeric_casts, unstable_name_collisions, unused_assignments, unused_import_braces, unused_imports, unused_must_use /* rustfmt::skip */
)]
#![deny(
    clippy::all, clippy::cargo, clippy::cast_lossless, clippy::clone_on_ref_ptr, clippy::cognitive_complexity, clippy::equatable_if_let, clippy::filetype_is_file, clippy::float_cmp_const, clippy::inefficient_to_string, clippy::iter_on_empty_collections, clippy::iter_on_single_items, clippy::linkedlist, clippy::macro_use_imports, clippy::manual_assert, clippy::manual_instant_elapsed, clippy::manual_string_new, clippy::match_like_matches_macro, clippy::match_wildcard_for_single_variants, clippy::mem_forget, clippy::needless_update, clippy::nursery, clippy::panic, clippy::pedantic, clippy::perf, clippy::string_add_assign, clippy::string_to_string, clippy::unnecessary_join, clippy::unnecessary_self_imports, clippy::unused_async, clippy::unwrap_used, clippy::verbose_file_reads, clippy::zero_sized_map_values /* rustfmt::skip */
)]
#![allow(clippy::module_name_repetitions)]
#![allow(
  clippy::bool_to_int_with_if, clippy::module_name_repetitions, clippy::multiple_crate_versions /* rustfmt::skip */
)]
#![allow(clippy::missing_const_for_fn)]
// #![cfg_attr(feature = "unstable", feature(ip))]
// The recursion_limit is mainly triggered by the json!() macro. The more key/value pairs there are the more recursion occurs. We want to keep this as low as possible, but not higher then 128. If you go above 128 it will cause rust-analyzer to fail,
#![recursion_limit = "94"]
// -------------------------------------------------------------------------------------------------------------------
// Uncomment while in dev mode.
//..
//.
// #![allow(unused)]

// extern crate dotenv;

mod db;
mod model;
mod route;
mod util;
// #[cfg(test)]
// mod test;

use crate::{db::DioDB, route::config};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 5000;
    dotenv().ok();

    println!("Starting server on PORT {PORT}");
    let db_client: Client = DioDB::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
