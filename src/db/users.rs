/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::users;

/* Database data structs (User, NewUser) */
use crate::models::users::NewUser;
use crate::models::users::User;

use diesel::result::Error;
use std::result::Result;

/* Get all our users as vector of User structs */
pub fn list() -> Vec<User> {
    users::table
        .select(users::all_columns)
        .load::<User>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!")
}

/* Insert our new user based on NewUser struct */
pub fn insert(new_user: NewUser) -> Result<usize, Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&crate::establish_connection())
}

/* Update our user based on id and NewUser struct */
pub fn update(id: i32, updated_user: NewUser) -> Result<usize, Error> {
    diesel::update(users::table.filter(users::id.eq(id)))
        .set(updated_user)
        .execute(&crate::establish_connection())
}

/* Delete a user based on id */
pub fn delete(id: i32) -> usize {
    diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(&crate::establish_connection())
        .expect("Ops, we can't delete this.")
}
