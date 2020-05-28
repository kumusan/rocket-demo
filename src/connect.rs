use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;

pub fn connect_init() -> PgConnection {
    dotenv().ok();

    let database_url = "postgres://postgres:postgres@localhost:5433";
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
