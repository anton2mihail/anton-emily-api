use rusqlite::{Connection, Result};

pub fn seed_db() -> Result<()> {
    let conn = Connection::open("./antonEmily.db")?;

    match conn.execute("DROP TABLE IF EXISTS user", ()) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("drop table user failed: {}", err),
    }

    match conn.execute(
        "create TABLE IF NOT EXISTS user (id INTEGER PRIMARY KEY AUTOINCREMENT, first_name TEXT, last_name TEXT, email TEXT NOT NULL, admin BOOLEAN NOT NULL)",
        (),
    ) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("create table user failed: {}", err),
    };

    match conn.execute(
        "create TABLE IF NOT EXISTS guest (id INTEGER PRIMARY KEY AUTOINCREMENT, first_name TEXT NOT NULL, last_name TEXT NOT NULL, email TEXT, phone_number TEXT, invite_sent BOOLEAN NOT NULL, given_password TEXT, invite_accepted BOOLEAN NOT NULL)",
        (),
    ) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("create table guest failed: {}", err),
    };


    match conn.execute(
        "INSERT INTO user (first_name, last_name, email, admin) values (?1, ?2, ?3, ?4)",
        ["Anton".to_string(), "Lachmaniucu".to_string(), "antonmihail@gmail.com".to_string(), true.to_string()],
    ) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("insert into user failed: {}", err),
    }

    match conn.execute(
        "INSERT INTO user (first_name, last_name, email, admin) values (?1, ?2, ?3, ?4)",
        ["Emily".to_string(), "Lachmaniucu".to_string(), "emilyvos124@gmail.com".to_string(), true.to_string()],
    ) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("insert into user failed: {}", err),
    };

    Ok(())
}
