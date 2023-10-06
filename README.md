# PUD-ASM

A personal project & programming language.

## About

PUD-ASM is a language that appears like an incredibly minimal, low level language. It is not.  
PUD-ASM is a esoteric programming language. It has practically zero use and functionality. It can't even print characters (not yet).

## Why

I needed a personal project, and wanted to do something with rust. I chose this.
My personal goal is to fully implement a sort of programming language, that uses single characters as keywords.

## pudc

The interpreter for PUD-ASM.
Designed to be as small as possible. Uses only rust std

### Building

Clone repo & `cargo build --release` and move the "pudc" file elsewhere, such as on your $PATH.

### Using

pudc accepts any file, but I like to use .psm as a file ending to organize things

To run a PUD-ASM file, simply run `pudc /path/to/file`

## Learning

All keywords are specified in the [main file](./src/main.rs) of the program. A basic doc can be invoked from the pudc compiler with `pudc doc`

Finally, an example program, which allocates two numbers, adds them and displays the result can be found [here](./test.psm)
