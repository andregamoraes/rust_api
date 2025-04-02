// src/models.rs
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

// Make User Insertable for the `users` table
#[derive(Insertable, Serialize, Deserialize )]
#[diesel(table_name = users)]  // Link this struct to the `users` table in the schema
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
