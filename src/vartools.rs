use std::process::Command;
use std::fmt;
use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use glob::glob;
use std::env;

use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Vartools {
    pub cmd: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool
}

impl Vartools {
    pub fn new(cmd: &str) -> Vartools {
        // make sure file and dir exists
        let cmd = if cmd.contains("vartools") { cmd } else { "" };
        Vartools {
            cmd: cmd.to_string(),
            stdout: "".to_string(),
            stderr: "".to_string(),
            success: false
        }
    }

    pub fn run(&mut self) -> bool {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                    .args(&["/C", &self.cmd])
                    .output()
                    .expect("failed to execute process")
        } else {
            Command::new("sh")
                    .arg("-c")
                    .arg(&self.cmd)
                    .output()
                    .expect("failed to execute process")
        };

        self.stdout = String::from_utf8_lossy(&output.stdout).to_string();
        self.stderr = String::from_utf8_lossy(&output.stderr).to_string();


        self.success = true;
        if self.stderr.len() > 0 {
            self.success = false;
        }

        self.success
    }
}

impl fmt::Display for Vartools {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.stdout.len() > 0 {
            write!(f, "{}", self.stdout)
        } else {
            write!(f, "{}", self.stderr)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_command() {
        let mut runner = Vartools::new("vartools -help");
        runner.run();
        assert!(!runner.success);
        assert!(runner.stderr.contains("Usage: vartools"));
    }

    #[test]
    fn test_that_it_wont_allow_not_a_vartools_command() {
        let result = Vartools::new("echo hello");
        assert_eq!(result.cmd, "");
        assert_eq!(result.stdout, "");
        assert_eq!(result.stderr, "");
    }
}
