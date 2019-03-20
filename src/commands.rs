// This module will take the vartools documentation (`vartools -listcommands`)
// and parse it into an object for use on the front end

use std::process::Command;
use std::collections::HashMap;
use regex::Regex;

use rocket_contrib::json::JsonValue;

fn get_docs_text() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "vartools -listcommands"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("vartools -listcommands")
            .output()
            .expect("failed to execute process")
    };

    String::from_utf8_lossy(&output.stderr).to_string()
}

pub fn parse_docs() -> JsonValue {
    let docs_text = get_docs_text();
    json!({"some": "object"})
}
