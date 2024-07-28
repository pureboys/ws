use std::{env, io};
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use crate::routers::{course_routers, general_routers};
use crate::state::AppState;

#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path= "../db_access/mod.rs"]
mod db_access;
#[path="../errors.rs"]
mod  errors;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = sqlx::PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_reponse: "I am OK".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });
    let app = move || {
        App::new().app_data(shared_data.clone()).configure(general_routers).configure(course_routers)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
