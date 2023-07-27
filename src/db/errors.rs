use std::fmt::Display;

use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::From;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(From, Debug)]
pub enum MyError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // match *self {
            // MyError::NotFound => "Not Found",   
            // MyError::PGError(_) => "PGError",
            // MyError::PGMError(_) => "PGMError",
            // MyError::PoolError(_) => "PoolError"
        // }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            MyError::PGError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            _ => {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
