use diesel;
use diesel::prelude::*;
use models::Todo;
use schema::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Todo>> {
    schema::diesel::todos::table.load::<Todo>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Todo> {
    todos::table.find(id).get_result::<Todo>(connection)
}

pub fn insert(Todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
    diesel::insert_into(todos::table)
        .values(&InsertableTodo::from_Todo(Todo))
        .get_result(connection)
}

pub fn update(id: i32, Todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
    diesel::update(todos::table.find(id))
        .set(&Todo)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(todos::table.find(id)).execute(connection)
}
