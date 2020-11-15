#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

/* Importing functions */
use diesel::mysql::MysqlConnection;
use diesel::Connection;
use dotenv::dotenv;
use rocket::response::content::Json;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::env;

mod db;
mod models;
mod routes;
pub mod schema;

/* This will return our mysql connection to use with diesel */
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/hello")]
fn hello() -> Template {
    #[derive(Serialize)]
    struct Context {
        name: String,
    }
    let context = Context {
        name: String::from("Patrique Ouimet"),
    };
    Template::render("index", &context)
}

#[get("/hello")]
fn api_hello() -> Json<&'static str> {
    Json("{\"status\": \"success\", \"message\": \"Hello API!\"}")
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/",
            routes![
                hello,
                routes::users::list,
                routes::users::new,
                routes::users::insert,
                routes::users::update,
                routes::users::process_update,
                routes::users::delete
            ],
        )
        .mount("/api", routes![api_hello])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
