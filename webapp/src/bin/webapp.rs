use std::env;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use tera::Tera;
use crate::wa::routers::app_config;

#[path = "../mod.rs"]
mod wa;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST_PORT must be set");
    println!("Listen on: {}", &host_port);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new().app_data(web::Data::new(tera)).configure(app_config)
    })
        .bind(&host_port)?
        .run()
        .await
}
