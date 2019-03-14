---
layout: page
title: Usage and Examples
---

This is an overview of the API exposed by `transits`. It's main purpose is to take some parameters, like a path to an input file, a vartools command, etc, then run vartools with those parameters. It will return the output of vartools and the data files that vartools creates.

Vartools works with very large files, and creates very large files. As such, the file data will not be sent through the web API. Instead, an absolute path to a file will be the input.
