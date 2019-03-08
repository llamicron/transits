#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};

mod runner;
use runner::Runner;

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    cmd: String,
    infile: String,
    outdir: String
}

#[post("/json", format = "application/json", data = "<payload>")]
fn handle_json(payload: Json<Payload>) -> JsonValue {
    let payload = payload.0;
    let mut r = Runner::new(&payload.cmd, &payload.infile, &payload.outdir);
    r.run();
    println!("{:?}", r);
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
    rocket().launch();
}
