/* Import macros and others */
use crate::schema::users;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser<'x> {
    pub name: &'x str,
}
