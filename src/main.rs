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
mod vartools;
mod plotter;

use plotter::DataPoint;
use vartools::Vartools;

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
    println!("Payload recieved.\n");
    let payload = payload;

    let df = match DataFormatter::new(&payload.infile) {
        Ok(x) => x,
        Err(e) => return json!({ "status": "error", "reason": format!("{}", e) }),
    };


    println!("Reformatting data...");
    df.reformat();
    println!("Done.\n");


    let mut vartools = Vartools::new(&payload.cmd);
    println!("Running Vartools... This could take some time...");
    vartools.run();

    let mut vartools_stdout_file = PathBuf::from(&df.vartools_path());
    vartools_stdout_file.push("parameters.txt");
    fs::write(vartools_stdout_file, format!("{}", vartools)).expect("Couldnt write to file");
    println!("Done. Vartools output written, stdout written to parameters.txt\n");

    println!("Drawing plots...");
    match plotter::plot_all_models_at(&df.vartools_path()) {
        Ok(_) => (),
        Err(e) => eprintln!("Error! Could not create one or more plot images"),
    };
    println!("Done.\n");

    println!("All tasks done.\n");
    json!({
        "status": "ok",
        "vartools": format!("{}", vartools),
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
    rocket().launch();
}
