use serde_derive::{Deserialize, Serialize};
use std::process::Command;


#[derive(Debug, Serialize, Deserialize)]
pub struct Runner {
    pub cmd: String,
    pub infile: String,
    pub outdir: String,
    pub stdout: String,
    pub success: bool
}

impl Runner {
    pub fn new(cmd: &str, infile: &str, outdir: &str) -> Runner {
        // make sure file and dir exists
        Runner {
            cmd: cmd.to_string(),
            infile: infile.to_string(),
            outdir: outdir.to_string(),
            stdout: "".to_string(),
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
        println!("{}", self.stdout);
        self.success = output.status.success();
        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_command() {
        let mut runner = Runner::new("echo hello", "/some/file/here", "/some/dir/here");
        runner.run();
        assert!(runner.success);
        assert_eq!(runner.stdout, "hello\n");
    }
}
