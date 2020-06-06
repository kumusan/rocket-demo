use crate::models::Todo;
use crate::query::*;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "hello wasm rocket!"
}

#[get("/todos")]
pub fn all_todos() -> Json<Vec<Todo>> {
    Json(all())
}

#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<Todo>) -> String {
    let todo: Todo = Todo{
        id: todo.id,
        title: todo.title.to_string(),
        body: todo.body.to_string(),
        done: todo.done,
    };
    create(todo);
    format!("ok")
}

#[get("/todos/<todoid>")]
pub fn get_id(todoid: i32) -> Json<Vec<Todo>> {
    let result = get(todoid);
    Json(result)
}

#[post("/todos/<todoid>", data = "<todo>")]
pub fn update_id(todoid: i32, todo: Json<Todo>) -> String {
    let todo: Todo = Todo{
        id: todo.id,
        title: todo.title.to_string(),
        body: todo.body.to_string(),
        done: todo.done,
    };
    update(todoid, todo);
    format!("ok")
}

#[delete("/todos/<todoid>")]
pub fn delete_id(todoid: i32) -> String {
    delete(todoid);
    format!("ok")
}

#[test]
fn route_test() {
    assert_eq!(index(), "hello wasm rocket!");
}
