use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use crate::routers::{course_routers, general_routers};
use crate::state::AppState;

#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../handler.rs"]
mod handler;
#[path = "../models.rs"]
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_reponse: "I am OK".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new().app_data(shared_data.clone()).configure(general_routers).configure(course_routers)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
