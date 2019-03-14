use std::fs;
use std::path::PathBuf;
use gnuplot::{Figure, Caption, Color};

pub enum DataPoint {
    MJD,
    MagObs,
    MagModel,
    Error,
    Phase
}

pub fn plot_model(file: &PathBuf, x: DataPoint, y: DataPoint) {
    if !file.exists() {
        // quit
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
    // fg.set_terminal("pngcairo", "/Users/llamicron/Desktop/test.png");
    fg.axes2d().points(xs, ys, &[Caption("MJD by Mag_obs"), Color("black")]);
    fg.show();
}

