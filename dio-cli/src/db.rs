//! [`model`] holds DATA STRUCTURES.

use crate::{util::idx_today, LEN_FACTS, LEN_PRINCIPLES};
use anyhow::{anyhow, Error, Result};
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Facts db data model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    /// `id` of the fact.
    pub id: u32,

    /// `title` or content of the fact.
    pub title: String,
}

/// Principle db data model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Principle {
    /// `id` of the principle.
    pub id: u32,

    /// `title` or content of the principle.
    pub title: String,
}

// -------------------------------------------------------------------------------------------------------------------

/// Server database structure.
/// TODO: What to do about `Fact` and `Principle`?
#[derive(Debug)]
pub struct DB {
    /// Facts db data model hashmap.
    pub facts: HashMap<String, String>,

    /// Principle db data model hashmap.
    pub principles: HashMap<String, String>,
}

/// Returns a today's fact or principle from the database.
///
/// Args:
///     db: A mutable reference to the database.
///
/// Returns:
///     A string containing a fact or principle.
///
/// # Panics
///
/// Panics if the database is empty.
pub fn get_todays_fact_or_principle(db: &mut DB) -> Result<String, Error> {
    let mut rng: ThreadRng = rand::thread_rng();

    // If random value is even return a fact, else return a principle.
    let result = if rng.gen_range(0..10) % 2 == 0 {
        db.facts.get(&idx_today(LEN_FACTS).to_string())
    } else {
        db.principles.get(&idx_today(LEN_PRINCIPLES).to_string())
    };

    match result {
        Some(r) => Ok(r.to_string()),
        None => Err(anyhow!("No result found")),
    }
}
