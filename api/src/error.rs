use actix_web::{error, Result};
use derive_more::{Display, Error, From};
use migration::DbErr;


#[derive(Debug, Display, From)]
pub enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    DbErr(DbErr),

    Error(String)
}
impl error::ResponseError for MyError {}

