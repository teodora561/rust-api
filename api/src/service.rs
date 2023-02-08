use entity::{prelude::{Cake, CakesModel, FruitsModel, CakesActiveModel, FruitsActiveModel, Fruit, FillingsModel, FillingsActiveModel, Filling}, cake};
use migration::sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, ActiveValue};
use serde_json::Value;

use crate::error::MyError;


pub async fn get_cakes_json(db: &DatabaseConnection) -> Result<Vec<(Value, Option<Value>)>, MyError> {
    let cakes = Cake::find().find_with_related(Fruit).into_json().all(db).await?;
    Ok(cakes)
}

pub async fn get_cakes(db: &DatabaseConnection) -> Result<Vec<(CakesModel, Option<FruitsModel>)>, MyError> {
    let cakes = Cake::find().find_also_related(Fruit).all(db).await?;
    Ok(cakes)
}

pub async fn get_cake_by_id(id: i32, db: &DatabaseConnection) -> Result<CakesModel, MyError> {
    let res = Cake::find_by_id(id).one(db).await
                                .map_err(MyError::DbErr)?
                                .ok_or("User not found".into())
                                .map_err(MyError::Error);
    
    res
}

pub async fn create_cake(db: &DatabaseConnection, val: Value) -> Result<CakesModel, MyError> {
    let mut json_cake = CakesActiveModel::from_json(val)?;
    json_cake.id = ActiveValue::NotSet;
    let cake = json_cake.insert(db).await?;
    Ok(cake)

}

pub async fn create_fruit(db: &DatabaseConnection, val: Value) -> Result<FruitsModel, MyError> {
    let json_fruit = FruitsActiveModel::from_json(val)?;
    let fruit = json_fruit.insert(db).await?;
    Ok(fruit)
}

pub async fn create_filling(db: &DatabaseConnection, val: Value) -> Result<FillingsModel, MyError> {
    let json_filling = FillingsActiveModel::from_json(val)?;
    let filling = json_filling.insert(db).await?;
    Ok(filling)
}