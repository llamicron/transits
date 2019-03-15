---
layout: page
title: Plotter
---

This module is responsible for creating graphs of the `.model` data from vartools. It uses [gnuplot](http://www.gnuplot.info/), so you'll need that installed.

# Usage
To graph a single model file:
```rust
// Plots MJD on the x-axis and MagObs on the y-axis
plot_by("/path/to/my/file.model", "/path/to/the/export/image.png", DataPoint::MJD, DataPoint::MagObs);

// Phase on the x-axis and MagModel on the y-axis
plot_by("/path/to/my/file.model", "/path/to/the/export/image.png", DataPoint::Phase, DataPoint::MagModel);

// ...
```

The outfile is optional. If a file is given, a png will be exported to that file location. If you want to display the graph and not immediately save it, pass `""` to the `outfile` parameter.

This `enum` contains the data types you can graph by. Pass one of these into the `x` and `y` fields in the methods and the plotter will find the appropriate data
```
pub enum DataPoint {
    MJD,
    MagObs,
    MagModel,
    Error,
    Phase
}
```

You can also **plot all model files in a directory**. This will create a `plots/` dir in the dir of model files. It returns a vec of the file names of the images
```rust
let images_created = plot_all_models_at(PathBuf::from("./models/are/here/")).expect("Could not create images");
```
