use std::fs;
use std::path::Path;
use std::io::Error;

#[derive(Debug)]
pub struct DataFormatter {
    pub infile: &'static str
}

// Methods
impl DataFormatter {
    fn check_file(&self) -> bool {
        Path::new(self.infile).is_file()
    }
}

// Associated functions
impl DataFormatter {
    pub fn new(infile: &'static str) -> Result<DataFormatter, &str> {
        let formatter = DataFormatter {
            infile
        };

        if formatter.check_file() {
            return Ok(formatter);
        }

        Err("File check failed, make sure the file exists and the path is correct")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_it_will_check_that_the_directory_exists() {
        let f = match DataFormatter::new("./some/in/file/that/doesnt/exist") {
            Ok(x) => x,
            Err(e) => panic!(e)
        };
    }

    #[test]
    fn it_will_succeed_on_a_real_file() {
        DataFormatter::new("./src/testdata/in/1.transit").expect("Could not find file");
    }

    #[test]
    #[should_panic]
    fn it_will_fail_on_a_dir() {
        match DataFormatter::new("./src/testdata") {
            Ok(_) => (),
            Err(e) => panic!(e)
        }
    }
}
