# Transits dev log

Warning: this is very unorganized, I'm just writing as I go along.

## Installation and Configuration
First thing I needed to do was install vartools and learn how to use it. I downloaded vartools to compile from source [(from here)](https://www.astro.princeton.edu/~jhartman/vartools.html). I tried to compile and install it with the command it provides:
```
$ ./configure && make && make install
```
This failed, saying it couldn't "compile a simple Fortran program". I installed `gcc` through homebrew with this:
```
$ brew update
$ brew install gcc
```
and then ran the above command again and it worked.
```
(python3) ~/code/experiments/vartools vartools
Usage: vartools <-i lcname  ["binary"] | -l lclist ["column" col] ["binary"]
["opencommand" command]>
    [-header] [-headeronly] [-tab]
    [-readformat Nskip ["stringid" colstringid] ["inpututc" format]
        col_time col_mag col_sig]
    [-inputlcformat var1:col1[:type1[:fmt1][,var2:col2[:vtype2[:fmt2]],...]]
        ["skipnum" Nskip] ["skipchar" chars]
        ["delimiter" delimiter]
    [-inlistvars var1:col1[:vtype1[:fmt1][,var2:col2[:vtyp2[:fmt2]],....]]]
    [-basename] [-readall] [-binaryperiodogram] [-jdtol jdtol] [-matchstringid]
    [-nobuffer] [-help ["all" | commandname]] [-quiet] [-randseed seed]
    [-bufferlines nlines] [-numbercolumns] [-listcommands [commandname]]
    [-redirectstats statsfile ["append"]] [-oneline]
    [-example commandname] [-showinputlistformat]
    [-showinputlcformat] [-log-command-line] [-skipmissing] [-noskipempty]
    [-parallel Nproc] [-functionlist]
    [-L libraryfile] [-F libraryfile] [-f functodefine]
    [-version]
    Command1... CommandN

    For a list of commands type:
        vartools -listcommands

    To view the syntax for a specific command type:
        vartools -listcommands $COMMANDNAME
    To see the syntax for the -LS command, for example, type:
        vartools -listcommands -LS

    To get detailed help type:
        vartools -help all

    For help on a particular command or option type:
        vartools -help $COMMANDNAME
    To see help on the -readformat option, for example, type:
        vartools -help -readformat

    To see an example use for a particular command type:
        vartools -example $COMMANDNAME

    VARTOOLS version 1.36

(python3) ~/code/experiments/vartools which vartools
/usr/local/bin/vartools
(python3) ~/code/experiments/vartools
```
#
# Using vartools
After reading through the [home site](https://www.astro.princeton.edu/~jhartman/vartools.html), I wasn't getting anywhere. I did some more searching and realized that the site linked above is more likely just a reference. I found a [publication](https://arxiv.org/pdf/1605.06811.pdf) by Hartman and Bakos, with detailed instructions on how to use vartools and what's going on behind the scenes.

From reading the paper, I learned that vartools is more of a pipeline than a new tool. There are lots of other software projects that can do the same thing as vartools. But vartools is best when piping data from one process to another. For other software projects, each step of data processing would usually read from a file and then write output to another file, repeating this process for each step. The speed of the computations began to be limited not by the speed of the processor but the read/write speed of the data drives. That's why vartools was build. You can pipe output from one command directly to another, leaving the data to be read once and kept in memory until you're completely done. This gives me an idea for a web interface, using blocks that you can construct to create a process, not unlike the procedures in my [brewkit](https://github.com/llamicron/brewkit.) project.

From vartools publication, section 2.1.2:

> When batch processing hundreds of thousands (or more) of individual light curve files, often the speed of the process is limited by attempts to read-from or write-to the hard disk, rather than by computation speed. In such cases there is a significant advantage to eliminating redundant attempts to access the disk. This is one advantage of writing a pipeline in Vartools, where a light curve is read into memory once and output to the disk only when explicitly requested

## Publication
Useful places:
* Section 2: Commands
* Section 3: Examples
    - Lomb-Scargle Example (s. 3.1; p. 19-21)

## Common Commands
* [input methods (`-i` and `-l` )](#input)
* [`-oneline`](#output)

## Input
One crucial command in using vartools is `-i` and `-l`. Basic usage of these is like so
```
$ vartools -i data/my_transit_data.transit ...
$ vartools -l data/my_transits_list ...
```
using `-i` will accept a path to a file with transit data in it (I take after Hartman and keep the `.transit` file ending, but it's just text).

Using `-l` will accept a path to a file, in which is a list of more transit data files. For example, a transit files list could contain:
```
data/my_first_transit.transit
data/my_second_transit.transit
data/my_third_transit.transit
...
```
This makes it easy to run batch processing for multiple transits.

#### Input Files
From the vartools publication (section 2.1.1) on input files

>A user may provide either a single light curve or an ascii list of light curve files as input to the program. Each light curve is stored in a separate file, either in ascii text format (one measurement per row, with separate whitespace delimited columns for the time, magnitude or flux, uncertainties, and other optional information), or as a bi-nary FITS table.

These files are whitespace delimited, so they can have spaces or tabs between each data point, like this:
```
53725.173920 10.1874 0.001210
53725.176540 10.1893 0.001490
53725.177720 10.1715 0.001280
...
```

### Output
A common command is `-oneline`, which will print data in a way that is more human-readable
