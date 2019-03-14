/*
 * This module is responsible for reformatting a data file
 * into a format that vartools can use. The format taken in is:
 *
 * starname,mjd,mag,error,mjd,mag,error,mjd,mag,error...
 * starname,mjd,mag,error,mjd,mag,error,mjd,mag,error...
 * starname,mjd,mag,error,mjd,mag,error,mjd,mag,error...
 * ...
 *
 * The starname is the first field in a line of CSV. The remaining fields are triplets of
 * modified julian date (mjd), magnitude (mag), and error. All data for one star is on one line
 * of the csv file. The number of fields in each line (*excluding* the starname) must be divisible
 * by 3, ie. none of the fields above can be missing.
 *
 * File can be comma delimited or space delimited
*/

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

const FILE_EXTENSION: &'static str = "obs";

#[derive(Debug)]
pub struct DataFormatter {
    pub infile: PathBuf
}

// Methods
impl DataFormatter {
    /// Rewrites data from infile to outdir
    /// takes a string of the relative or absolute path of the output directory
    /// Returns a vector of files written
    pub fn reformat_to(&self, outpath: &mut PathBuf) -> Result<Vec<PathBuf>, Error> {

        outpath.push(&self.infile.file_stem().unwrap());
        match fs::create_dir_all(&outpath) {
            Err(e) => return Err(e),
            Ok(_) => ()
        };

        let mut files_written: Vec<PathBuf> = vec![];

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
                // TODO test this
                return Err(Error::new(ErrorKind::Other, "Error in data format, an uneven number of datapoints is present (not divisible by 3)"));
            }

            let mut i = 0;
            let mut to_write = String::new();
            while i < stardata.len() {
                let chunk = format!("{} {} {}\n", stardata[i], stardata[i + 1], stardata[i + 2]);
                to_write.push_str(chunk.as_str());
                i += 3;
            }

            let mut filepath = PathBuf::new();
            filepath.push(&outpath);
            filepath.push(starname);
            filepath.set_extension(&FILE_EXTENSION);

            println!("File {} written to disk", fs::canonicalize(&filepath).unwrap().display());
            fs::write(&filepath, &to_write)?;
            files_written.push(filepath);
        }

        self.write_index_file(&outpath, &files_written);

        Ok(files_written)
    }

    fn write_index_file(&self, outdir: &PathBuf, files: &Vec<PathBuf>){
        let mut index_content = String::new();
        for file in files {
            index_content.push_str(file.file_name().unwrap().to_str().unwrap());
            index_content.push_str("\n");
        }

        let mut index_file = PathBuf::new();
        index_file.push(&outdir);
        index_file.push("lc_list");
        match fs::canonicalize(&index_file) {
            Ok(x) => {
                println!("File {} written to disk (index)", &x.display());
                fs::write(&x, &index_content).expect("Could not write to file");
            },
            Err(e) => panic!("Could not write index file!")
        }
    }
}

// Associated functions
impl DataFormatter {
    /// Takes a string of an absolute or relative path to the input
    pub fn new(infile: &'static str) -> Result<DataFormatter, Error> {
        let mut inputfile = PathBuf::new();
        inputfile.push(infile);

        if Path::new(infile).is_file() {
            return Ok(DataFormatter {
                infile: inputfile
            });
        }

        Err(Error::new(ErrorKind::Other, "File check failed. Make sure file exists"))
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
        let f = DataFormatter::new("./src/testdata/in/example.dat").expect("Couldnt find file");
        let mut outdir = PathBuf::new();
        outdir.push("./src/testdata/out/");
        match f.reformat_to(&mut outdir) {
            Ok(_) => println!("Reformatting complete"),
            Err(_) => assert!(false)
        };

        assert!(Path::new("./src/testdata/out/example").is_dir());
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
