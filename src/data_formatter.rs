use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::time::SystemTime;
use serde::Deserialize;

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

use csv::ReaderBuilder;

const FILE_EXTENSION: &'static str = "obs";

#[derive(Debug)]
pub struct DataFormatter {
    pub infile: PathBuf,
    pub path: PathBuf
}

#[derive(Deserialize)]
struct DataPoint {
    starname: String,
    mjd: String,
    mag: String,
    magerror: String,
}

// Methods
impl DataFormatter {

    fn create_needed_dirs(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.formatted_path())?;
        fs::create_dir_all(&self.vartools_path())?;
        Ok(())
    }

    /// Rewrites data from infile to outdir
    /// takes a string of the relative or absolute path of the output directory
    /// Creates an 'lc_list' file for vartools to use (index of other files)
    pub fn reformat(&self) -> Result<(), Error> {

        let start = SystemTime::now();

        self.create_needed_dirs()?;

        let contents = fs::read_to_string(&self.infile)?;
        let mut rdr = ReaderBuilder::new().from_reader(contents.as_bytes());

        for row in rdr.records() {
            let record = row?;
            let data: DataPoint = record.deserialize(None)?;


            let mut filename = self.formatted_path();
            filename.push(format!("{}.obs", data.starname));

            let file = OpenOptions::new()
                                    .append(true)
                                    .create(true)
                                    .open(filename).unwrap();

            let mut buf = BufWriter::new(file);
            writeln!(buf, "{},{},{}", data.mjd, data.mag, data.magerror)?;
        }
        self.write_index_file();

        println!("Finished reformatting in {:?} seconds", SystemTime::now().duration_since(start).unwrap());
        Ok(())
    }

    fn write_index_file(&self) -> bool {
        let mut index_content = String::new();

        for entry in fs::read_dir(&self.formatted_path()).unwrap() {
            let file = entry.unwrap();
            if file.path().to_str().unwrap().contains("lc_list") {
               continue;
            }
            index_content.push_str(file.path().to_str().unwrap());
            index_content.push_str("\n");

        }
        match fs::write(format!("{}/lc_list", &self.formatted_path().display()), &index_content) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    // path/formatted_input/
    pub fn formatted_path(&self) -> PathBuf {
        let mut fp = PathBuf::from(&self.path);
        fp.push("formatted_input");
        fp
    }

    // path/vartools/
    pub fn vartools_path(&self) -> PathBuf {
        let mut vp = PathBuf::from(&self.path);
        vp.push("vartools");
        vp
    }
}

// Associated functions
impl DataFormatter {
    /// Takes a string of an absolute or relative path to the input
    pub fn new(infile: &str) -> Result<DataFormatter, Error> {
        if !Path::new(infile).exists() {
            return Err(Error::new(ErrorKind::NotFound, "File does not exist"));
        }

        // /some/dir/october.dat
        let inputfile = PathBuf::from(infile);

        // /some/dir/october/
        let mut path = PathBuf::from(&infile).parent().unwrap().to_owned();
        path.push(&inputfile.file_stem().unwrap());


        if inputfile.exists() {
            let df = DataFormatter {
                infile: inputfile,
                path
            };
            return Ok(df);
        }

        Err(Error::new(ErrorKind::NotFound, "File check failed. Make sure file exists"))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn test_it_will_check_that_the_directory_exists() {
//         let f = match DataFormatter::new("./some/in/file/that/doesnt/exist") {
//             Ok(x) => x,
//             Err(e) => panic!(e)
//         };
//     }

//     #[test]
//     fn it_will_succeed_on_a_real_file() {
//         let f = DataFormatter::new("./src/testdata/in/example.dat").expect("Couldnt find file");
//         let mut outdir = PathBuf::new();
//         outdir.push("./src/testdata/out/");
//         f.reformat();

//         assert!(Path::new("./src/testdata/out/example").is_dir());
//     }

//     #[test]
//     #[should_panic]
//     fn it_will_fail_on_a_dir() {
//         match DataFormatter::new("./src/testdata") {
//             Ok(_) => (),
//             Err(e) => panic!(e)
//         }
//     }
// }
