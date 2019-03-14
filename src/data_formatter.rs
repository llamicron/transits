use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

const FILE_EXTENSION: &'static str = "obs";

#[derive(Debug)]
pub struct DataFormatter {
    pub infile: PathBuf,
    pub path: PathBuf
}

// Methods
impl DataFormatter {
    /// Rewrites data from infile to outdir
    /// takes a string of the relative or absolute path of the output directory
    /// Returns a vector of files written
    pub fn reformat(&self) -> bool {

        match fs::create_dir_all(&self.formatted_path()) {
            Err(e) => return false,
            Ok(_) => ()
        };

        match fs::create_dir_all(&self.vartools_path()) {
            Err(e) => return false,
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
                println!("Star data not divisible by 3");
                return false;
            }

            let mut i = 0;
            let mut to_write = String::new();
            while i < stardata.len() {
                let chunk = format!("{} {} {}\n", stardata[i], stardata[i + 1], stardata[i + 2]);
                to_write.push_str(chunk.as_str());
                i += 3;
            }

            fs::write(format!("{}/{}.{}", &self.formatted_path().display(), &starname, &FILE_EXTENSION), &to_write).expect("Couldnt write to file");
        }

        self.write_index_file();
        true
    }

    fn write_index_file(&self) -> bool {
        let mut index_content = String::new();

        for entry in fs::read_dir(&self.formatted_path()).unwrap() {
            let file = entry.unwrap();
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
        match f.reformat(&mut outdir) {
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
