#![feature(box_syntax, custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod error;
pub mod models;
pub mod schema;
pub mod service;