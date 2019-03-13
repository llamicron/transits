use std::fs;
use std::path::{Path, PathBuf};
use std::io::Error;

#[derive(Debug)]
pub struct DataFormatter {
    pub infile: &'static str
}

// Methods
impl DataFormatter {
    /// Rewrites data from infile to outdir
    /// Returns a vector of files written
    pub fn reformat_to(&self, outdir: &str) -> Result<Vec<PathBuf>, &str> {
        if !Path::new(outdir).is_dir() {
            return Err("outdir doesn't exist");
        }

        let mut files_written = vec![];

        let contents = fs::read_to_string(&self.infile).expect("File could not be read");

        // Each line gets written to it's own file
        for line in contents.split("\n").collect::<Vec<&str>>() {
            if line.len() <= 0 {
                continue;
            }

            let mut split_ch = " ";
            if line.contains(",") {
                split_ch = ","
            }

            let mut stardata = line.split(&split_ch).collect::<Vec<&str>>();

            let starname = stardata.remove(0);

            if stardata.len() % 3 != 0 {
                return Err("Error in data format, an uneven number of datapoints is present (not divisible by 3)");
            }

            let mut i = 0;
            let mut to_write = String::new();
            while i < stardata.len() {
                let chunk = format!("{} {} {}\n", stardata[i], stardata[i + 1], stardata[i + 2]);
                to_write.push_str(chunk.as_str());
                i += 3;
            }

            let mut path = PathBuf::new();
            path.push(outdir);

            let mut given_path = PathBuf::new();
            given_path.push(self.infile);
            path.push(&given_path.file_stem().unwrap());

            fs::create_dir_all(&path);

            path.push(format!("{}.transit", starname));

            fs::write(&path, &to_write).expect("Couldn't write to file");
            files_written.push(path);
        }

        Ok(files_written)
    }
}

// Associated functions
impl DataFormatter {
    pub fn new(infile: &'static str) -> Result<DataFormatter, &str> {
        let formatter = DataFormatter {
            infile
        };

        if Path::new(infile).is_file() {
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
