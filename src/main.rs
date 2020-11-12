#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::response::content::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/")]
fn hello() -> Template {
    #[derive(Serialize)]
    struct Context { name: String }
    let context = Context { name: String::from("Patrique Ouimet") };
    Template::render("index", &context)
}

#[get("/hello")]
fn api_hello() -> Json<&'static str> {
  Json("{\"status\": \"success\", \"message\": \"Hello API!\"}")
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![hello])
        .mount("/api", routes![api_hello])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
