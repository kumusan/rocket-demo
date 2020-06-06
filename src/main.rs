#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod connect;
pub mod models;
pub mod router;
pub mod schema;
pub mod query;
use router::*;

fn main() {
    connect::connect_init();
    rocket::ignite()
        .mount("/", routes![index, all_todos, new_todo, get_id])
        .launch();
}
