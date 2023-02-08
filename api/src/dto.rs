use std::fmt::Display;

use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CakeDto {
    pub name: String,
}

impl Display for CakeDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FruitDto {
    pub name: String,
    pub cake_id: i32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FillingDto {
    pub name: String,
    pub cake_id: i32
}