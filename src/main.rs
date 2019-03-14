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

mod data_formatter;
mod runner;
mod plotter;

use plotter::DataPoint;
use runner::Runner;

use std::path::PathBuf;
use std::fs;

use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};
use data_formatter::DataFormatter;

use gnuplot::{Figure, Caption, Color};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VartoolsPayload {
    cmd: String,
    infile: String
}

#[post("/vartools", format = "application/json", data = "<payload>")]
fn handle_json(payload: Json<VartoolsPayload>) -> JsonValue {
    let payload = payload;


    let df = match DataFormatter::new(&payload.infile) {
        Ok(x) => x,
        Err(e) => {
            return json!({
                "status": "error",
                "reason": format!("{}", e)
            });
        }
    };


    df.reformat();

    let mut r = Runner::new(&payload.cmd);
    r.run();

    let mut vartools_stdout_file = PathBuf::from(df.vartools_path());
    vartools_stdout_file.push("parameters.txt");
    fs::write(vartools_stdout_file, format!("{}", r)).expect("Couldnt write to file");

    json!({
        "status": "ok",
        "vartools": format!("{}", r),
    })
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
    let df = DataFormatter::new("/Users/llamicron/Desktop/october.dat").expect("fuck");
    df.reformat();
}
