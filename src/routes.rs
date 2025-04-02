use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{NewUser, User};
use crate::schema::users;
use crate::DbPool;
use serde::Deserialize;
use diesel::insert_into;
use diesel::query_dsl::RunQueryDsl;

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>
}

#[get("/users")]
async fn get_users(query: web::Query<QueryParams>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let user_list: Vec<User> = match &query.name {
        Some(name) => {
            users::table
                .filter(users::name.eq(name))
                .load::<User>(&mut conn)
                .expect("Error loading users by name")
        }
        None => {
            users::table
                .load::<User>(&mut conn)
                .expect("Error loading all users")
        }
    };

    HttpResponse::Ok().json(user_list)
}

#[post("/users")]
async fn create_user(user: web::Json<NewUser>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let new_user = user.into_inner();

    let inserted_user: User = insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
        .expect("Error inserting user");

    HttpResponse::Created().json(inserted_user)
}
