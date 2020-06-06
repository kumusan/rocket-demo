use diesel::*;
use crate::models::*;
use crate::schema::todos::dsl::*;
use crate::connect::connect_init;
use diesel::pg::Pg;

pub fn all() -> Vec<Todo> {
    let connection = connect_init();
    todos.load::<Todo>(&connection).unwrap()
}

pub fn get(todoid: usize) -> Todo {
    let connection = connect_init();
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let todoid = todoid - 1;
    db_load[todoid].clone()
}

pub fn create(todo: Todo) {
    let connection = connect_init();
    insert_into(todos).values(todo).execute(&connection).unwrap();
}

// pub fn update(id: i32, Todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
//     diesel::update(todos::table.find(id))
//         .set(&Todo)
//         .get_result(connection)
// }

// pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(todos::table.find(id)).execute(connection)
// }

#[test]
fn query() {
    insert_query_test();
    get_all();
    select_test();
    delete_test();
}

fn insert_query_test() {
    let query = insert_into(todos).default_values();
    let sql = "INSERT INTO \"todos\" DEFAULT VALUES -- binds: []";
    assert_eq!(sql, debug_query::<Pg, _>(&query).to_string());
}

fn get_all() { 
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

fn select_test() {
    let connection = connect_init();
    let db_load = todos.filter(id.eq(1)).load(&connection).unwrap();
    let test_todo = vec![Todo {id: 1, title: "new title".to_string(), body: "new text".to_string(), done: false}];
    assert_eq!(test_todo, db_load, "db_load is []");
    let test_todo = Todo::new(1, "new title", "new text", false);
    assert_eq!(Ok(test_todo), todos.filter(id.eq(1)).first(&connection));
}

fn delete_test() {
    let connection = connect_init();
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let id_1 = db_load[0].clone();
    let id_2 = db_load[1].clone();
    delete(&id_2).execute(&connection).unwrap();
    assert_eq!(Ok(vec![id_1]), todos.load(&connection));
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let id_1 = db_load[0].clone();
    delete(&id_1).execute(&connection).unwrap();
}
