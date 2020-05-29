#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod connect;
mod models;
mod router;
mod schema;
mod query;
use router::*;

fn main() {
    connect::connect_init();
    rocket::ignite()
        .mount("/", routes![index, todos, new_todo])
        .launch();
}

// db操作できるならpostでtodo追加
// getでlist返す
// modelsに型追加？

// id指定でselect

// フロント側はめんどくさいからやらない
