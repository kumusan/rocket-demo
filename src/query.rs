use diesel::*;
use crate::models::*;
// use crate::schema::todos;
use crate::connect::connect_init;
#[cfg(test)]
use diesel::debug_query;
// use diesel::select;

mod schema {
    table! {
    pub todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        done -> Bool,
    }
}
}

#[test]
pub fn query_test() { //-> Result<QueryResult<Vec<Todo>>, Err> {
    // todos::table.load::<Todo>(&*connection)
    use schema::todos::dsl::*;
    let connection = connect_init();
    connection.execute("SELECT * FROM todos").unwrap();
    assert_eq!(
        Ok(Todo::new(1, " new title", "new text")), 
        todos.find(1).first(&connection));
}
// pub fn all(connection: &PgConnection) {
//     let results = todos
//         .filter(done.eq(true))
//         .limit(5)
//         .load::<Todo>(&connection)
//         .expect("Err all");
// }

// pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Todo> {
//     connection.execute("SELECT * FROM todos WHERE ")
// }

// pub fn insert(Todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
//     diesel::insert_into(todos::table)
//         .values(&Todo::from_todo(Todo))
//         .get_result(connection)
// }

// pub fn update(id: i32, Todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
//     diesel::update(todos::table.find(id))
//         .set(&Todo)
//         .get_result(connection)
// }

// pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(todos::table.find(id)).execute(connection)
// }
