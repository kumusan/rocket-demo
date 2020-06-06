use diesel::*;
use crate::models::*;
use crate::schema::todos::dsl::*;
use crate::connect::connect_init;
use diesel::pg::Pg;

pub fn all() -> Vec<Todo> {
    let connection = connect_init();
    todos.load::<Todo>(&connection).unwrap()
}

pub fn get(todoid: i32) -> Vec<Todo> {
    let connection = connect_init();
    todos.filter(id.eq(todoid)).load(&connection).unwrap()
}

pub fn create(todo: Todo) {
    let connection = connect_init();
    diesel::insert_into(todos).values(todo).execute(&connection).unwrap();
}

pub fn update(todoid: i32, todo: Todo) {
    let connection = connect_init();
    diesel::update(todos.filter(id.eq(todoid)))
        .set(&todo)
        .execute(&connection)
        .unwrap();
}

pub fn delete(todoid: i32) {
    let connection = connect_init();
    diesel::delete(todos.filter(id.eq(todoid))).execute(&connection).unwrap();
}

#[test]
fn query() {
    insert_query_test();
    get_all();
    select_test();
    update_test();
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
    assert_eq!(test_todo, db_load);
    let test_todo = Todo::new(1, "new title", "new text", false);
    assert_eq!(Ok(test_todo), todos.filter(id.eq(1)).first(&connection));
}

fn update_test() {
    let connection = connect_init();
    let changes = Todo {
        id: 1,
        title: "change title".to_string(),
        body: "change body".to_string(),
        done: false,
    };
    diesel::update(todos.filter(id.eq(1)))
        .set(&changes)
        .execute(&connection)
        .unwrap();
    let db_load: Vec<Todo> = todos.filter(id.eq(1)).load(&connection).unwrap();
    let test_todo = Todo::new(1, "change title", "change body", false);
    assert_eq!(test_todo, db_load[0]);
}

fn delete_test() {
    let connection = connect_init();
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let id_1 = db_load[0].clone();
    let id_2 = db_load[1].clone();
    diesel::delete(&id_2).execute(&connection).unwrap();
    assert_eq!(Ok(vec![id_1]), todos.load(&connection));
    let db_load = todos.load::<Todo>(&connection).unwrap();
    let id_1 = db_load[0].clone();
    diesel::delete(&id_1).execute(&connection).unwrap();
}
