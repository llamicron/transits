---
layout: page
title: DataFormatter
---

# Overview

The `DataFormatter` module is responsible for translating given data to a format that vartools can read.

## Paths
A directory will be created with the file stem of the data file passed in. Two subdirectories will be created: `vartools` and `formatted_input`. The `vartools` directory will contain vartools output and another directory, `plots/`, of plot images. `formatted_input` will contain the input files, reformatted to work with vartools

## Input
Input files need to be in a specific format. See the [format](#format) section below for specific details. Input files contain the observation data for many stars in one file. It is passed to the DataFormatter and is split into smaller files. When referencing an input file, use a full, absolute path to the file.

**Note!** The data file **must** have a file extension. I recommend `.dat`.

## Output
As mentioned in the [paths](#paths) section above, output from this program will be written to a directory with the same file stem (excluding the extension) as the input file. Data will be further sorted in that directory. See the tree below for an illustration.

```
.
├── october.dat
└── october
   ├── formatted_input
   │   ├── examplestar.obs
   │   ├── examplestar2.obs
   │   ├── examplestar3.obs
   │   └── lc_list
   └── vartools
       ├── examplestar.obs.bls
       ├── examplestar.obs.bls.model
       ├── examplestar.obs.bls.phcurve
       ├── examplestar2.obs.bls
       ├── examplestar2.obs.bls.model
       ├── examplestar2.obs.bls.phcurve
       ├── examplestar3.obs.bls
       ├── examplestar3.obs.bls.model
       ├── examplestar3.obs.bls.phcurve
       ├── parameters.txt
       └── plots
           ├── examplestar.obs.bls_mjd_by_magobs.png
           ├── examplestar.obs.bls_phs_by_magmod.png
           ├── examplestar2.obs.bls_mjd_by_magobs.png
           ├── examplestar2.obs.bls_phs_by_magmod.png
           ├── examplestar3.obs.bls_mjd_by_magobs.png
           └── examplestar3.obs.bls_phs_by_magmod.png
```

As you can see, there are quite a few files in the output directory. Let's go through them.

* The `formatted_output/` directory contains the input data, rewritten for vartools. It also contains `lc_list`, which is an index contianing all the light curves to analyze.
* The vartools directory contains all output from vartools.
  * `.bls` for each star in the input data
  * `.bls.phcurve` for each star
  * `.bls.model` for each star
  * `parameters.txt` holds the `stdout` or `stderr` from vartools
  * `plots/` holds two graphs for each star. Plots are built from the `.model` files above


## Formats

### Input file
The given data format for the input file is as follows:

```
starname, mjd, mag, err, mjd, mag, err, mjd, mag, err, ...
starname, mjd, mag, err, mjd, mag, err, mjd, mag, err, ...
starname, mjd, mag, err, mjd, mag, err, mjd, mag, err, ...
...
```

Each line contains observed data for a single star. Data comes in groups of 3:
```
mjd = modified julian date
mag = magnitude
err = error
```
**If any of these are missing, the program will fail**. The amount of data points, exluding the starname, must be divisible by 3.

### `star.obs`
These files are in the `formatted_input/` directory. They are created by the DataFormatter to be used by vartools. Each `.obs` file contains the data from one line of the input file. The format of a `.obs` file is simply:
```
mjd mag err
mjd mag err
mjd mag err
...
```

# Usage and Examples
```rust
// This can be a relative path, but absolute paths are preffered.
let df = match DataFormatter::new("/Users/llamicron/Desktop/october.dat").expect("something went wrong...");
f.reformat()
```
