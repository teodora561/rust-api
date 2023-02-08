use actix_web::{web, App, middleware, HttpServer};
use migration::{sea_orm::Database, Migrator, MigratorTrait};
mod controller;
mod error;
mod service;
mod dto;

#[actix_web::main] 
async fn start() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    //let host = std::env::var("HOST").expect("HOST");
    //let port = std::env::var("PORT").expect("PORT");

    let conn = Database::connect(&database_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = web::Data::new(conn);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default()) // enable logger
            .service(controller::index)
            .service(controller::create_cake)
            .service(controller::get_cake)
            .service(controller::create_fruit)
            .service(controller::get_cakes)
            .service(controller::get_cakes_json)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
