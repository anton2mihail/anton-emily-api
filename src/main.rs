#[macro_use] extern crate rocket;
use crud::User;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rusqlite::{params, Connection, Error};
use std::sync::Mutex;
use serde::Deserialize;
use rocket::serde::json::serde_json;

mod seed;
mod crud;

#[get("/")]
fn index() -> (Status, &'static str) {
    (Status::Ok, "ok!")
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserLogin<'r> {
    email: &'r str,
    password: &'r str,
}

#[post("/login", data = "<payload>")]
fn login(payload: Json<UserLogin<'_>>, cookies: &CookieJar<'_>, db: &State<Mutex<Connection>>) -> (Status, &'static str) {


    cookies.add_private(("name", "value"));

    (Status::Ok, "ok!")
}

#[get("/users")]
fn users(cookies: &CookieJar<'_>, db: &State<Mutex<Connection>>) -> (Status, String) {
    let users_from_db = crud::get_users(db).unwrap();
    println!("Users from db: {:?}", users_from_db);
    let _ = users_from_db.iter().map(|user| println!("{:?}", user));

    let json = serde_json::to_string(&users_from_db).unwrap();
    (Status::Ok, json)
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>, db: &State<Mutex<Connection>>) -> Flash<Redirect> {
    cookies.remove_private("user_id");
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[launch]
fn rocket() -> _ {
    let conn = Connection::open("antonEmily.db").unwrap();

    let seeding_result = match seed::seed_db() {
        Ok(_) => true,
        Err(error) => panic!("Unable to Seed Database {}", error),
    };

    if seeding_result {
        println!("Database Seeded Successfully");
    }

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![login])
        .mount("/", routes![users])
        .mount("/", routes![logout])
        .manage(Mutex::new(conn))
}
