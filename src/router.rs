use rocket_contrib::json::Json;
use crate::models::Todo;

#[get("/")]
pub fn index() -> &'static str {
    "hello wasm rocket!"
}

#[get("/todos")]
pub fn todos() -> Json<Vec<Todo>> {
  Json(vec![Todo{
    id: 1,
    title: "test1".into(),
    description: "これはテストです".into(),
    done: false,
  }])
}

#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<Todo>) -> String {
  format!("post {:?}", todo.0)
}

// #[get("/todos/<id>")]
// pub fn get_id(id: u32) -> String {
//   let todo = Todo {
//     id: id,
//   };
//   format!("{:?}, todo")
// }
