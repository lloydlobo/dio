//! 'model' contain MongoDB database models.

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// The snippet above does the following:
///
/// Imports the required dependencies
/// Uses the derive macro to generate implementation support for formatting the output, serializing, and deserializing the data structure.
/// Creates a User struct with required properties. We also added field attributes to the id property to rename and ignore the field if it is empty.
/// PS: The pub modifier makes the struct and its property public and can be accessed from other files/modules.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Facts {
    /// MONGODB UUID Auto gen.
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub _id: Option<ObjectId>,

    /// Index of the item.
    pub id: i32,

    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Principles {
    /// MONGODB UUID Auto gen.
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub _id: Option<ObjectId>,

    /// Index of the item.
    pub id: i32,

    pub title: String,
}
