use diesel::*;
use crate::models::*;
use crate::schema::todos::dsl::*;
use crate::connect::connect_init;
#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
use diesel::pg::Pg;

#[test]
pub fn insert_test() {
    let query = insert_into(todos).default_values();
    let sql = "INSERT INTO \"todos\" DEFAULT VALUES -- binds: []";
    assert_eq!(sql, debug_query::<Pg, _>(&query).to_string());
}

#[test]
pub fn get_all() { //-> Result<QueryResult<Vec<Todo>>, Err> {
    let connection = connect_init();
    // insert_into(todos).values(Todo::new(1, "new title", "new text", false)).execute(&connection).unwrap();
    // すでにinsertしているためErrが返ってくる
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let test_todo = vec![Todo { id: 1, title: "new title".to_string(), body: "new text".to_string(), done: false }];
    assert_eq!(test_todo, db_load);
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
