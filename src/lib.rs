#![feature(box_syntax, plugin, question_mark)]
#![plugin(dotenv_macros)]

extern crate dotenv;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;

pub mod model;
pub mod service;