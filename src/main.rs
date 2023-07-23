#![allow(dead_code)]
extern crate dotenv;

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use schema::*;
use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    let mut conn = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

    create(&mut conn);
    finish(&mut conn);
}

fn create(conn: &mut PgConnection) {
    let new_log = NewTodo {
        text: "A new todo is created".to_string(),
    };

    let inserted_row = diesel::insert_into(todos::table).values(&new_log).get_result::<Todo>(conn);
    
    println!("{:?}", inserted_row);
}

fn finish(conn: &mut PgConnection) {
    let todos = todos::dsl::todos.filter(todos::done.eq(false).and(todos::id.eq(1)));

    let updated_row = diesel::update(todos)
        .set((
            todos::done.eq(true),
            todos::finish_timestamp.eq(Some(chrono::Utc::now())),
        ))
        .get_result::<Todo>(conn);

    println!("{:?}", updated_row);
}