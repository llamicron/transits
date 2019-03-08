// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
// extern crate serde_derive;

// use rocket_contrib::json::{Json, JsonValue};

mod runner;
use runner::Runner;


// #[post("/json", format = "application/json", data = "<runner>")]
// fn handle_json(_runner: Json<Runner>) -> JsonValue {
//     json!({ "status": "ok" })
// }

// #[catch(404)]
// fn not_found() -> JsonValue {
//     json!({
//         "status": "error",
//         "reason": "Resource was not found."
//     })
// }

// fn rocket() -> rocket::Rocket {
//     rocket::ignite()
//         .mount("/", routes![handle_json])
//         .register(catchers![not_found])
// }

fn main() {
    // rocket().launch();
    let mut r = Runner::new("echo hello there", "/some/file", "/some/dir");
    print!("Before running: ");
    println!("{}", r.stdout);
    r.run();
    print!("After running: ");
    println!("{}", r.stdout);
}
