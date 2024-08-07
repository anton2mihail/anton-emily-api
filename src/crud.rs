use rusqlite::{Connection, Result};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rocket::State;
use std::sync::Mutex;
use serde::Deserialize;
use serde::Serialize;

pub fn create_guest(first_name: String, last_name: String, email: String, phone_number: String) -> Result<()> {
    let conn = Connection::open("antonEmily.db")?;

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    conn.execute(
        "INSERT INTO guest (first_name, last_name, email, phone_number, invite_sent, given_password, invite_accepted) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [first_name.to_string(), last_name.to_string(), email.to_string(), phone_number.to_string(), false.to_string(), rand_string.to_string(), false.to_string()],
    )?;


    Ok(())
}

pub fn authenticate_user() -> Result<()> {
    let conn = Connection::open("antonEmily.db")?;

    Ok(())
}

pub fn authenticate_guest() -> Result<()> {
    let conn = Connection::open("antonEmily.db")?;

    Ok(())
}


pub fn update_guest() -> Result<()> {
    let conn = Connection::open("antonEmily.db")?;

    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

pub fn get_users(db: &State<Mutex<Connection>>) -> Result<Vec<User>> {
    let conn = db.lock().expect("shared state lock");

    let mut stmt = conn.prepare("SELECT id, first_name, last_name, email FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            email: row.get(3)?,
        })
    }).unwrap().map(|f| f.unwrap()).collect();

    Ok(user_iter)
}
