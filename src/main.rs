#![feature(proc_macro_hygiene, decl_macro)]
// Remove these for production
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;
extern crate glob;

mod data_formatter;
mod runner;
use runner::Runner;

use std::path::PathBuf;

use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};
use data_formatter::DataFormatter;


#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    cmd: String
}

#[post("/json", format = "application/json", data = "<payload>")]
fn handle_json(payload: Json<Payload>) -> JsonValue {
    let payload = payload.0;
    let mut r = Runner::new(&payload.cmd);
    r.run();
    println!("{}", r);
    json!(r)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![handle_json])
        .register(catchers![not_found])
}

fn main() {
    // rocket().launch();
    let f = DataFormatter::new("./src/testdata/in/example.dat").expect("Couldnt find file");

    let mut outdir = PathBuf::new();
    outdir.push("./src/testdata/out/");

    let result = f.reformat_to(&mut outdir).expect("Something went wrong");
}
