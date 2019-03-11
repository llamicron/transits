use std::process::Command;
use std::fmt;
use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use glob::glob;
use std::env;

use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Runner {
    pub cmd: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool
}

impl Runner {
    pub fn new(cmd: &str) -> Runner {
        // make sure file and dir exists
        let cmd = if cmd.contains("vartools") { cmd } else { "" };
        Runner {
            cmd: cmd.to_string(),
            stdout: "".to_string(),
            stderr: "".to_string(),
            success: false
        }
    }

    pub fn list_files<'a>(&self, path: Path) -> io::Result<Vec<Path>> {
        let mut path = env::current_dir().expect("Can't access current directory");
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

impl fmt::Display for Runner {
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
        let mut runner = Runner::new("vartools -help");
        runner.run();
        assert!(!runner.success);
        assert!(runner.stderr.contains("Usage: vartools"));
    }

    #[test]
    fn test_that_it_wont_allow_not_a_vartools_command() {
        let mut result = Runner::new("echo hello");
        assert_eq!(result.cmd, "");
        assert_eq!(result.stdout, "");
        assert_eq!(result.stderr, "");
    }

    #[test]
    fn test_list_files() {
        let mut r = Runner::new("vartools -h");
        println!("{:?}", r.list_files("data/in/"));
    }
}
