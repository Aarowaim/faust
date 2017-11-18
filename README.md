# Faust
## An Interpreter for a Diabolical Language
Just in case you haven't had your fill of brainf\*\*\*, here's my humble interpreter.
It's fast enough for practical use, but is still a WIP.
A few more levels of optimization and support for other brainf\*\*\* are in the works.

So far there are a few things that I'm quite pleased with:

- It contains a benchmarking macro that was hard-won from studying the esoteric macro language within rust.
- It doesn't use much recursion or nesting, so it is stack-safe, even with very large brainf\*\*\* programs.
- It executes code from both files and user input.

Erik Bosman's infamous [mandelbrot.b](http://esoteric.sange.fi/brainfuck/utils/mandelbrot/) pushes interpreters to their limit.
The sheer complexity is a ripe target for optimizations, and in fact the program was written with significant help in the form of a higher-level brainf\*\*\*.

## `benchmark!` results in release mode
- Once upon a time, mandelbrot.b interpreted character-by-character took *26956892 milliseconds* (*7.5 hours*) to finish.
- mandelbrot.b takes only *26767 milliseconds* (*27 seconds*) after a commit which condenses repeated commands together.
- Currently, mandelbrot.b takes only *17247 milliseconds* (*17 seconds*), thanks to optimizing the decoding order a little bit; `[-]` appears much less often than `[` and `]`. The reason this change is so effective is that interpreters make cpu branch prediction difficult, so correctly ordering the short-circuiting is an effective way to compensate.

## Goals
My primary goal is to execute mandelbrot.b within 10 seconds using only code transformations.

A secondary goal is to support the many variants of brainf\*\*\* using frontends defined in a YAML file.
Among the smaller goals is support for brainf\*\*\* [synonyms](http://esolangs.org/wiki/TrivialBrainfuckSubstitution) such as [Ook!](http://esolangs.org/wiki/Ook!).
An interactive debugger and some threading support would be awesome.

With the knowledge this project brings, I hope to make a toy language and other potentially useful interpreters.

## Cloning
### If you use git
    mkdir foo
    cd foo
    git clone https://github.com/Aarowaim/faust.git

### If not, no worries
Download the repository from github as a zip and extract it to a folder

## Building
### Cargo
- Go to the folder **above** `src`. Then type `cargo build --release` in a terminal at that folder.
- You may now run using `cargo run --release` followed by an optional file containing brainf\*\*\* code. If no arguments are given you will later be prompted for a line of input to be executed as brainf\*\*\* code. 
- Run any of the examples from the example folder by calling `cargo run --release examples\<filename>`. To run mandelbrot, which will be my benchmark, the command is `cargo run --release examples\mandelbrot.b`. Try running `rot13.b` or `sierpinski.b` if you don't want to wait so long.

### I don't use cargo
- `rustc src\main.rs` will produce a target directory containing the executable `faust_brainfuck`.
- Executing `faust_brainfuck` with filepath as a terminal argument will run that file
