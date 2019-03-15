---
layout: default
title: Home
---

# Transits
## A VARTOOLS pipeline

This project aims to simplify the vartools process by providing a pipeline with simple inputs and useful outputs.

### What is VARTOOLS?
[VARTOOLS](https://www.astro.princeton.edu/~jhartman/vartools.html) (or just vartools) is a program for analyzing astronomical light curves. It was written by [Joel Hartman](https://www.astro.princeton.edu/~jhartman/) at Princeton, supported by a NASA grant. It provides a large number of tools to analyze light curves, and provides a way to string together any number of commands with large sets of data.


### What is this program?
This program was created because vartools command can get very tedious to write. There are also some functions that vartools does not do, like graphing. Faculty at the University of Dallas use vartools to analyze light curves, but some additional work is needed. This program will automate much of that additional work, with vartools at the center.

**In a nutshell**, this program takes a data file in a [specified format](https://llamicron.github.io/transits/data_formatter), reformats that data to a more verbose format that vartools uses, and hands it off to vartools. It then creates a safe directory and pipes all of vartools output to files in that directory. After vartools is done, this program looks for `.model` files in the vartools output, and creates `png` graphs of them.

So far, this program is only a web API with no outward-facing interface. Soon, and interface will be built that will allow a user to quickly and easily create a vartools command to send to the API.
