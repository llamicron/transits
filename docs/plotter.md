---
layout: page
title: Plotter
---

This module is responsible for creating graphs of the `.model` data from vartools. It uses [gnuplot](http://www.gnuplot.info/), so you'll need that installed.

# Usage
This module currently contains one `enum` and one method. The `enum` is simply:
```
pub enum DataPoint {
    MJD,
    MagObs,
    MagModel,
    Error,
    Phase
}
```
to differentiate between the types of data in the `.model` file.

The method signature is this:
```
pub fn plot_by(infile: &str, outfile: &str, x: DataPoint, y: DataPoint) {
    ...
}
```
The infile is a `&str` of a path to a `.model` file, part of the vartools output. The outfile is optional. If a file is given, a png will be exported to that file location. If you want to display the graph and not immediately save it, pass `""` to the `outfile` parameter.

`x` and `y` are `enum` values as above. You can choose which of the two values from the `.model` you want to graph, and what axis you want them on. For example, to graph julian date (MJD) on the x-axis and observed magnitude (MagObs) on the y-axis:
```rust
use plotter::plot_by;
use plotter::DataPoint;

// MJD by MagObs
plot_by("/path/to/model/file.bls.model", "", DataPoint::MJD, DataPoint::MagObs);

// Model by Phase
plot_by("/path/to/model/file.bls.model", "", DataPoint::MagModel, DataPoint::Phase);
```
