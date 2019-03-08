#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Command {
    cmd: String,
    optional: bool
}

#[post("/json", format = "application/json", data = "<command>")]
fn handle_json(command: Json<Command>) -> JsonValue {
    json!({ "status": "ok" })
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
