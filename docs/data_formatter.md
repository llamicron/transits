---
layout: page
title: DataFormatter
---

# Overview

The `DataFormatter` module is responsible for translating given data to a format that vartools can read.

## Input
The given data format is as follows:

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

## Output
Open providing a path to a data file with the above format, the `DataFormatter` will rewrite the data into several files.

You will need to provide an output directory. To prevent loose files through an accidental path error, another directory will be created inside the output directory provided. This directory will have the name of the input file provided (excluding the extension). For example:

```
.
├── in
│   └── october_data.dat
└── out
    └── october_data
        ├── star1.obs
        ├── star2.obs
        ├── star3.obs
        └── ...

```

One file will be written to the output directory **for each star**, with the `.obs` extension. Each of these files will contain the data points from the observed star's line in the input file. It will be reformatted as a space separated file (vartools likes spaces, maybe because it's astronomy).

```
$ cat in/october_data.dat
my_star, mdj1, mag1, err1, mjd2, mag2, err2, ...
...

$ cat out/october_data/my_star.obs
mdj1 mag1 err1
mjd2 mag2 err2
...
```

# Usage and Examples
```rust
// This can be a relative path
let f = DataFormatter::new("./data/in/example.dat").expect("Couldnt find file");

// Outdir needs to be a PathBuf
let mut outdir = PathBuf::new();
outdir.push("./data/out/");

// reformat_to returns a vec of all files written to disk
let result = f.reformat_to(&mut outdir).expect("Something went wrong");
```
