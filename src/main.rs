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
use std::fs;

use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};
use data_formatter::DataFormatter;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct VartoolsPayload {
    cmd: String,
    infile: String
}

#[post("/vartools", format = "application/json", data = "<payload>")]
fn handle_json(payload: Json<VartoolsPayload>) -> JsonValue {
    let payload = payload;

    let mut outdir = PathBuf::from(&payload.infile);
    outdir = match outdir.parent() {
        Some(path) => PathBuf::from(path),
        None => outdir
    };

    let df = match DataFormatter::new(&payload.infile) {
        Ok(x) => x,
        Err(e) => {
            return json!({
                "status": "error",
                "reason": format!("{}", e)
            });
        }
    };


    let files_written = match df.reformat_to(&mut outdir) {
        Ok(x) => x,
        Err(e) => return json!({
            "status": "error",
            "reason": format!("{}", e)
        })
    };

    let mut r = Runner::new(&payload.cmd);
    r.run();

    let mut vartools_stdout_file = PathBuf::from(&outdir);
    vartools_stdout_file.push("vartools/parameters.txt");
    fs::write(vartools_stdout_file, format!("{}", r));

    json!({
        "status": "ok",
        "vartools": format!("{}", r),
        "files_written": files_written
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
    // let mut outdir = PathBuf::new();
    // outdir.push("/Users/llamicron/code/transits/src/testdata/out");

    // let df = DataFormatter::new("/Users/llamicron/code/transits/src/testdata/in/example.dat").expect("Something went wrong");
    // df.reformat_to(&mut outdir);
}
