#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

mod models;
mod router;

use router::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, todos, new_todo])
        .launch();
}

// db操作できるならpostでtodo追加
// getでlist返す
// modelsに型追加？

// id指定でselect

// フロント側はめんどくさいからやらない