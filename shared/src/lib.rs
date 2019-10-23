#[macro_use]
extern crate diesel;
extern crate juniper;
extern crate r2d2;

pub mod database;
pub mod error;
pub mod logger;
pub mod models;
pub mod schema;
pub mod utils;

// TODO: Where to put this?
pub struct Context {
    pub pool: database::PgPool,
}

impl juniper::Context for Context {}
