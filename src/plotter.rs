use std::fmt;
use std::fs;
use std::path::PathBuf;
use gnuplot::{Figure, Caption, Color, LegendOption, LabelOption, AlignType, Coordinate};

pub enum DataPoint {
    MJD,
    MagObs,
    MagModel,
    Error,
    Phase,
}

impl fmt::Display for DataPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           DataPoint::MJD      => write!(f, "MJD"),
           DataPoint::MagObs   => write!(f, "MagObs"),
           DataPoint::MagModel => write!(f, "MagModel"),
           DataPoint::Error    => write!(f, "Error"),
           DataPoint::Phase    => write!(f, "Phase"),
       }
    }
}

pub fn plot_by(infile: &str, outfile: &str, x: DataPoint, y: DataPoint) {
    let file = PathBuf::from(infile);

    if !file.exists() {
        panic!("File does not exist!")
    }

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    let data = fs::read_to_string(file).expect("Couldnt read data file for plotting");
    let lines = data.split("\n").collect::<Vec<&str>>();

    for line in lines {
        if line.len() <= 0 || line.contains("#") {
            continue;
        }

        let values = line.split(" ").collect::<Vec<&str>>();

        let x_value = match x {
            DataPoint::MJD      => values[0],
            DataPoint::MagObs   => values[1],
            DataPoint::MagModel => values[2],
            DataPoint::Error    => values[3],
            DataPoint::Phase    => values[4],
        };
        xs.push(x_value.parse().unwrap());

        let y_value = match y {
            DataPoint::MJD      => values[0],
            DataPoint::MagObs   => values[1],
            DataPoint::MagModel => values[2],
            DataPoint::Error    => values[3],
            DataPoint::Phase    => values[4],
        };
        ys.push(y_value.parse().unwrap());

    }
    let mut fg = Figure::new();

    if outfile.len() > 0 {
        let out_path = PathBuf::from(outfile);
        fg.set_terminal("pngcairo", &out_path.to_str().unwrap());
    }
    fg.axes2d().points(xs, ys, &[Caption(&format!("{} (x) by {} (y)", x, y)), Color("black")]);
    fg.show();
}

