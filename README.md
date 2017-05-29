# Faust
## An Awful Interpreter for an Awful Language
Just in case you haven't had your fill of brainfuck, here's my humble interpreter.
It is a learning milestone for me, so I promise it's not going to be noteworthy. 

So far there are a few things that I'm quite pleased with:

- It contains a benchmarking macro that was hard-won from studying the esoteric macro language within rust.
- It executes code from both files and user input.
- It is legible.

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
