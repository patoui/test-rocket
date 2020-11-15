/* To be able to return Templates */
use rocket_contrib::templates::Template;
use std::collections::HashMap;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::users;

/* Database data structs (User, NewUser) */
use crate::models::User;
use crate::models::NewUser;

/* To be able to parse raw forms */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Flash message and redirect */
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

/* List our inserted users */
#[get("/")]
pub fn list(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    /* Get all our users from database */
    let users: Vec<User> = users::table
        .select(users::all_columns)
        .load::<User>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");

    /* Insert on the template rendering
    context our new users vec */
    if let Some(ref msg) = flash {
        context.insert("data", (users, msg.msg()));
    } else {
        context.insert("data", (users, "Listing users..."));
    }

    /* Return the template */
    Template::render("list", &context)
}

#[get("/new")]
pub fn new(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("new", context)
}

#[post("/insert", data = "<user_data>")]
pub fn insert(content_type: &ContentType, user_data: Data) -> Flash<Redirect> {
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("name"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, user_data, options);

    match multipart_form_data {
        Ok(form) => {
            /* Insert our form data inside our database */
            let insert = diesel::insert_into(users::table)
                .values(NewUser {
                    name: match form.texts.get("name") {
                        Some(value) => &value[0].text,
                        None => "No Name.",
                    },
                })
                .execute(&crate::establish_connection());

            match insert {
                Ok(_) => Flash::success(
                    Redirect::to("/"),
                    "Success! We got a new User on our database!",
                ),
                Err(err_msg) => Flash::error(
                    Redirect::to("/new"),
                    format!(
                        "Houston, We had problems while inserting things into our database ... {}",
                        err_msg
                    ),
                ),
            }
        }
        Err(err_msg) => {
            /* Falls to this patter if theres some fields that isn't allowed or bolsonaro rules this code */
            Flash::error(
                Redirect::to("/new"),
                format!(
                    "Houston, We have problems parsing our form... Debug info: {}",
                    err_msg
                ),
            )
        }
    }
}

#[get("/update/<id>")]
pub fn update(id: i32) -> Template {
    let mut context = HashMap::new();
    let user_data = users::table
        .select(users::all_columns)
        .filter(users::id.eq(id))
        .first::<User>(&crate::establish_connection())
        .expect("Something happned while retrieving the User of this id");

    context.insert("User", user_data);

    Template::render("update", &context)
}

#[post("/update", data = "<user_data>")]
pub fn process_update(content_type: &ContentType, user_data: Data) -> Flash<Redirect> {
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("id"),
        MultipartFormDataField::text("name"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, user_data, options);

    match multipart_form_data {
        Ok(form) => {
            /* Insert our form data inside our database */
            let insert = diesel::update(
                users::table.filter(
                    users::id.eq(form.texts.get("id").unwrap()[0]
                        .text
                        .parse::<i32>()
                        .unwrap()),
                ),
            )
            .set(NewUser {
                name: match form.texts.get("name") {
                    Some(value) => &value[0].text,
                    None => "No Name.",
                },
            })
            .execute(&crate::establish_connection());

            match insert {
                Ok(_) => Flash::success(
                    Redirect::to("/"),
                    "Success! We got a new User on our database!",
                ),
                Err(err_msg) => Flash::error(
                    Redirect::to("/new"),
                    format!(
                        "Houston, We had problems while inserting things into our database ... {}",
                        err_msg
                    ),
                ),
            }
        }
        Err(err_msg) => {
            /* Falls to this patter if theres some fields that isn't allowed or bolsonaro rules this code */
            Flash::error(
                Redirect::to("/new"),
                format!(
                    "Houston, We have problems parsing our form... Debug info: {}",
                    err_msg
                ),
            )
        }
    }
}

#[get("/delete/<id>")]
pub fn delete(id: i32) -> Flash<Redirect> {
    diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(&crate::establish_connection())
        .expect("Ops, we can't delete this.");
    Flash::success(Redirect::to("/"), "Yey! The User was deleted.")
}
