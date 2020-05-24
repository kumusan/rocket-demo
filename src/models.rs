use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
  pub id: u32,
  pub title: String,
  pub description: String,
  pub done: bool,
}
