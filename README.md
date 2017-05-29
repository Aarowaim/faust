# Faust
## An Awful Interpreter for an Awful Language
Just in case you haven't had your fill of brainfuck, here's my humble interpreter.
It is a learning milestone for me, so I promise it's not going to be noteworthy. 

So far there are a few things that I'm quite pleased with:

- It contains a benchmarking macro that was hard-won from studying the esoteric macro language within rust.
- It executes code from both files and user input.
- It is legible.
- It doesn't use recursion, so it is stack-safe, even with very large brainfuck programs.

Erik Bosman's infamous [mandelbrot.b](http://esoteric.sange.fi/brainfuck/utils/mandelbrot/) pushes interpreters to their limit.
The sheer complexity is a ripe target for optimizations, and in fact the program was written with significant help in the form of a higher-level brainfuck.

`benchmark!` shows that mandelbrot.b takes *26956892 milliseconds* or *7.5 hours* to finish. In `--release` mode. *\*shiver\**

Trivial programs execute within milliseconds.

## Goals
My primary goal is to execute mandelbrot.b within 10 seconds using only code transformations.

A secondary goal is to support the many variants of brainfuck using frontends defined in a YAML file.
In particular, debugger and threading support would be awesome.
Among the smaller goals is support for brainfuck [synonyms](http://esolangs.org/wiki/TrivialBrainfuckSubstitution) such as [Ook!](http://esolangs.org/wiki/Ook!).

With the knowledge this project brings, I hope to make a toy language and other potentially useful interpreters.

## How to Compile & Run
1. clone the github repository to a local directory using `git clone https://github.com/Aarowaim/faust.git` or by downloading the repository as a zip and extracting it.
2. Go to the extracted directory and use rust's cargo: `cargo build --release`.
3. You may now run it using `cargo run --release`, followed by an optional file containing brainfuck code. If no arguments are given you will later be prompted for a line of input to be executed as brainfuck code. You may run any of the examples from the example folder by calling `cargo run --release examples\<filename>`. To run mandelbrot, which will be my benchmark, the command is `cargo run --release examples\mandelbrot.b`. Try running `rot13.b` or `sierpinski.b` if you don't want to wait so long.
4. If you choose to compile without cargo, `rustc` will produce a target directory containing the executable `faust_brainfuck`.
