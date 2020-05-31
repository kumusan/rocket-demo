use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
  pub id: u32,
  pub title: String,
  pub body: String,
  pub done: bool,
}

impl Todo {
  pub fn new(id: u32, title: &str, body: &str) -> Self {
    Todo {
        id: id,
        title: title.to_string(),
        body: body.to_string(),
        done: false,
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
