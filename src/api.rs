// Standard libs
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs;

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
#[get("/running")]
fn running() -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[get("/input_files", format = "application/json")]
fn get_input_files() -> JsonValue {
    // This is just for development
    let mut files: Vec<_> = fs::read_dir("/Users/llamicron/code/astrotools_data/").unwrap().map(|res| res.unwrap().path()).collect();
    // Production
    // let mut files: Vec<_> = fs::read_dir("/home/data").unwrap().map(|res| res.unwrap().path()).collect();
    files.sort();

    json!({
        "status": "ok",
        "files": json!(files)
    })
}


// #[post("/file_exists", format = "application/json", data = "<payload>")]
// fn file_exists(payload: Json<HashMap<&str, &str>>) -> JsonValue {
//     if !&payload.0.contains_key("file_path") {
//         return json!({
//             "status": "error",
//             "file_exists": false,
//             "reason": "no entry found for key. payload needs key 'file_path'"
//         })
//     }

//     json!({
//         "status": "ok",
//         "file_exists": PathBuf::from(&payload.0["file_path"]).is_file()
//     })
// }


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
    match plot_all_models_at(&df.vartools_path()) {
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

    #[test]
    fn file_exists() {
        let client = client();

        // Doesn't accept a get request
        let mut result = client.get("/api/file_exists").dispatch();
        assert_eq!(result.status(), Status::NotFound);

        // Doesn't accept a payload without the key "file_exists"
        let mut result = client.post("/api/file_exists")
                               .body(r#"{}"#)
                               .header(ContentType::JSON)
                               .dispatch();

        assert!(result.body_string().unwrap().contains("no entry found for key"));

        let mut result = client.post("/api/file_exists")
                               .body(r#"{"file_path":"/file/doesnt/exist"}"#)
                               .header(ContentType::JSON)
                               .dispatch();

        let result_string = match result.body_string() {
            Some(x) => x,
            _ => String::from("failed")
        };

        assert!(result_string.contains("ok"));
        assert!(result_string.contains("false"));
    }
}
