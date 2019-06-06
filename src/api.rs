// Standard libs
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs;
use std::env;

// External libs
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

use serde_derive::{Deserialize, Serialize};
use serde_json;

// My libs
use crate::plotter::{DataPoint, plot_all_models_at};
use crate::vartools::Vartools;
use crate::data_formatter::DataFormatter;

use gnuplot::{Figure, Caption, Color};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VartoolsPayload {
    cmd: String,
    infile: String
}


// Routes
// Sanity check
#[get("/running")]
fn running() -> JsonValue {
    json!({
        "status": "ok"
    })
}

// Returns a list of input files in the data directory (/home/data)
#[get("/input_files", format = "application/json")]
fn get_input_files() -> JsonValue {

    let path = match env::var("ASTROTOOLS_DEV") {
        Ok(_) => "/Users/llamicron/code/astrotools_data/",
        Err(_) => "/home/data"
    };

    println!("Using data directory {:?}", path);

    let files: Vec<_> = fs::read_dir(path).unwrap().map(|res| res.unwrap().path()).collect();

    let mut culled_files = files.iter().filter(|file| match file.extension() {
        Some(extension) => extension == "dat",
        None => false
    }).collect::<Vec<_>>();

    culled_files.sort();

    json!({
        "status": "ok",
        "files": json!(culled_files)
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

    vartools.cmd = str::replace(&vartools.cmd, "{infile}", &format!("{}/lc_list", &df.formatted_path().to_str().unwrap())[..]);
    vartools.cmd = str::replace(&vartools.cmd, "{outdir}", &df.vartools_path().into_os_string().into_string().unwrap());

    println!("Running Vartools... This could take some time...");
    vartools.run();

    let mut vartools_stdout_file = PathBuf::from(&df.vartools_path());
    vartools_stdout_file.push("parameters.txt");
    fs::write(vartools_stdout_file, format!("{}", vartools)).expect("Couldnt write to file");
    println!("Done. Vartools output written, stdout written to parameters.txt\n");

    println!("Drawing plots...");
    match plot_all_models_at(&df.vartools_path()) {
        Ok(_) => (),
        Err(e) => eprintln!("Error! Could not create one or more plot images"),
    };
    println!("Done.\n");

    println!("All tasks finished.\n");
    json!({
        "status": "ok",
        "vartools": format!("{}", vartools),
    })
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json!({
        "status": "error",
        "reason": "400 Bad request"
    })
}

#[catch(422)]
fn unprocessable() -> JsonValue {
    json!({
        "status": "error",
        "reason": "422 Unprocessable Entity"
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "404 Resource was not found."
    })
}

pub fn api() -> rocket::Rocket {
    let cors = rocket_cors::CorsOptions::default().to_cors().expect("Could not create CORS defaults");
    rocket::ignite()
        .mount("/api", routes![vartools, running, get_input_files])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/interface")))
        .attach(cors)
        .register(catchers![not_found, bad_request, unprocessable])
}






#[cfg(test)]
pub mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;

    fn client() -> Client {
        Client::new(api()).expect("valid rocket instance")
    }

    #[test]
    fn test_404_catcher() {
        let cl = client();
        let result = cl.get("/api/doesnt/exist").dispatch();
        assert_eq!(result.status(), Status::NotFound);
    }

    #[test]
    fn is_running() {
        let cl = client();
        let mut result = cl.get("/api/running").dispatch();
        assert_eq!(result.status(), Status::Ok);

        assert!(result.body_string().unwrap().contains("ok"));
    }
}
