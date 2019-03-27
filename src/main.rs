#![feature(proc_macro_hygiene, decl_macro)]
// Remove these for production
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate gnuplot;
extern crate serde_derive;
extern crate glob;
extern crate rocket_cors;
extern crate regex;

mod api;
mod data_formatter;
mod vartools;
mod plotter;

use api::api;

fn main() {
    api().launch();
}
