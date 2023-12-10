<!--
Template: https://gist.github.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc
-->

# conlogger

A simple connection status logger

## About

conlogger is a small program that logs whenever your devices loses internet connection (as long as the program is running).  
The resulting log file contains the exact interval in which you had a connection (or lost it) and the duration of said interval in seconds.   
I was motivated to create this program after years of a *very good experience with a certain [Internet prodiver](https://en.wikipedia.org/wiki/Vodafone_Germany).*

## Installing

Grab the [latest release](https://github.com/SlenLen/conlogger/releases/latest) from the releases. The executable will create the log files in the same directory it is in, so make sure you have write permission for the directory.

I only provide Windows x86-64 and Linux x86-64 executables. If you have a different CPU architecture or use a different OS, see below for compiling instructions.

## Building
Simply clone the repository with `git clone https://github.com/SlenLen/conlogger.git` and run `cargo build --release`.

### Built with
- [clap](https://github.com/clap-rs/clap) for command line arguments
- [chrono](https://github.com/chronotope/chrono) for getting time and date
