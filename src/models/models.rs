use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub user_email: &'a str,
    pub user_password: &'a str,
}


#[derive(Deserialize)]
pub struct CreateUser {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}

#[derive(Deserialize)]
pub struct QueryUser {
    pub user_name: String,
}

#[derive(Serialize)]
pub struct RequestResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_name: String,
    pub user_email: String,
}