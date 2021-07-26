#![recursion_limit = "256"]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

pub mod db;
pub mod game_data;
pub mod model;
mod schema;
