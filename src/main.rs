#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

pub mod models;
pub mod connect;
pub mod router;
use router::*;

fn main() {
    connect::init_pool();
    rocket::ignite()
        .mount("/", routes![index, todos, new_todo])
        .launch();
}

// db操作できるならpostでtodo追加
// getでlist返す
// modelsに型追加？

// id指定でselect

// フロント側はめんどくさいからやらない