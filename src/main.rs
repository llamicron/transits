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

mod data_formatter;
mod vartools;
mod plotter;

// Standard libs
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs;

// External libs
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};

// My libs
use plotter::DataPoint;
use vartools::Vartools;
use data_formatter::DataFormatter;


use gnuplot::{Figure, Caption, Color};



#[derive(Debug, Clone, Serialize, Deserialize)]
struct VartoolsPayload {
    cmd: String,
    infile: String
}

#[get("/running")]
fn running() -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[post("/file_exists", format = "application/json", data = "<payload>")]
fn file_exists(payload: Json<HashMap<&str, &str>>) -> JsonValue {
    // println!("{:?}", payload.0);
    // println!("{:?}", PathBuf::from(&payload.0["file_path"]).is_file());
    json!({
        "status": "ok",
        "file_exists": PathBuf::from(payload.0["file_path"]).is_file()
    })
}

#[post("/vartools", format = "application/json", data = "<payload>")]
fn vartools(payload: Json<VartoolsPayload>) -> JsonValue {
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
    let cors = rocket_cors::CorsOptions::default().to_cors().expect("Could not create CORS defaults");
    rocket::ignite()
        .mount("/api", routes![vartools, running, file_exists])
        .attach(cors)
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
