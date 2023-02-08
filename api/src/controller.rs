
use actix_web::{get, web::{self, Data}, Responder, post, HttpResponse, Error};
use migration::{sea_orm::{DatabaseConnection}, Iden};
use entity::{prelude::Cake};
use crate::{service, dto::{CakeDto, FruitDto, FillingDto}};

#[get("/hello/{name}")]
pub async fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!!!")
}

#[post("/cakes")]
pub async fn create_cake(db:Data<DatabaseConnection>, cake: web::Json<CakeDto>) -> Result<HttpResponse, Error> {
    let c = serde_json::value::to_value(cake.clone())?;
    let cake = service::create_cake(&db, c).await?;
    Ok(HttpResponse::Ok().json(cake))
}

#[get("/cakes/{id}")]
pub async fn get_cake(db: Data<DatabaseConnection>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let cake = service::get_cake_by_id(*id, &db).await?;
    Ok(HttpResponse::Ok().json(cake))
}

#[get("/cakes")]
pub async fn get_cakes(db: Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let cakes = service::get_cakes(&db).await?;
    Ok(HttpResponse::Ok().json(cakes))
}

#[get("/cakesjson")]
pub async fn get_cakes_json(db: Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let cakes = service::get_cakes_json(&db).await?;
    Ok(HttpResponse::Ok().json(cakes))
}

#[post("/fruits")]
pub async fn create_fruit(db: Data<DatabaseConnection>, fruit: web::Json<FruitDto>) -> Result<HttpResponse, Error> {
    let fruit = serde_json::value::to_value(fruit)?;
    let fruit = service::create_fruit(&db, fruit).await?;
    Ok(HttpResponse::Ok().json(fruit))
}

#[post("/fillings")]
pub async fn create_fillings(db: Data<DatabaseConnection>, filling: web::Json<FillingDto>) -> Result<HttpResponse, Error> {
    let filling = serde_json::value::to_value(filling)?;
    let filling = service::create_fruit(&db, filling).await?;
    Ok(HttpResponse::Ok().json(filling))
}

