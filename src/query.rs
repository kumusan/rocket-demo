use diesel::*;
use crate::models::*;
use crate::schema::todos::dsl::*;
use crate::connect::connect_init;
#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
use diesel::pg::Pg;

#[test]
pub fn insert_query_test() {
    let query = insert_into(todos).default_values();
    let sql = "INSERT INTO \"todos\" DEFAULT VALUES -- binds: []";
    assert_eq!(sql, debug_query::<Pg, _>(&query).to_string());
}

#[test]
pub fn get_all() { 
    let connection = connect_init();
    insert_into(todos)
        .values(
            vec![
                Todo::new(1, "new title", "new text", false), 
                Todo::new(2, "test2", "test2", false)
            ]
        )
        .execute(&connection).unwrap();
    // ローカルならErrが返ってくる
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let test_todo = vec![
        Todo {id: 1, title: "new title".to_string(), body: "new text".to_string(), done: false},
        Todo {id: 2, title: "test2".to_string(), body: "test2".to_string(), done: false},
    ];
    assert_eq!(test_todo, db_load);
}

#[test]
pub fn select_test() {
    let connection = connect_init();
    insert_into(todos)
        .values(
            vec![
                Todo::new(1, "new title", "new text", false), 
                Todo::new(2, "test2", "test2", false)
            ]
        )
        .execute(&connection).unwrap();
    let select = todos.select((id, title, body, done));
    let db_load = select.load(&connection).unwrap();
    let test_todo = vec![Todo {id: 1, title: "new title".to_string(), body: "new text".to_string(), done: false}];
    assert_eq!(test_todo, db_load);
}

// pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Todo> {
//     connection.execute("SELECT * FROM todos WHERE ")
// }

pub fn insert(todo: Todo) {
    let connection = connect_init();
    insert_into(todos).values(todo).execute(&connection);
}

// pub fn update(id: i32, Todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
//     diesel::update(todos::table.find(id))
//         .set(&Todo)
//         .get_result(connection)
// }

// pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(todos::table.find(id)).execute(connection)
// }
