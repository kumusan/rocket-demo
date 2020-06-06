// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Queryable)]
use diesel::*;
use super::schema::todos;
use serde::{Deserialize, Serialize};

#[cfg(feature = "postgres")]
include!("schema.rs");

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Deserialize, Serialize
)]
#[table_name = "todos"]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub done: bool,
}

impl Todo {
  pub fn new(id: i32, title: &str, body: &str, done: bool) -> Self {
    Todo {
        id: id,
        title: title.to_string(),
        body: body.to_string(),
        done: done,
    }
  }
  fn from_todo(todo: Todo) -> Todo {
    Todo {
      id: todo.id,
      title: todo.title,
      body: todo.body,
      done: todo.done,
    }
  }
}
