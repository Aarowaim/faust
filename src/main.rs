use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read};

#[macro_use] mod benchmark;

mod frontend;
mod interpreter;

fn get_input_line() -> String {    
    let mut input = String::new();
    println!();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    String::from(input.trim())
}

fn read_file_as_string<P: AsRef<Path>>(path: P) -> String {
    let f = File::open(path);

    let mut input = String::new();
    f.expect("Error reading file.").read_to_string(&mut input).unwrap();
    String::from(input.trim())
}

fn welcome() -> String {
    println!("Faust Brainfuck Interpreter\n");

    let args: Vec<_> = env::args().collect();
    
    if args.len() > 1 {
        println!("Reading file...");
    	read_file_as_string(&args[1])
    } else {
    	print!("Enter a brainfuck program:");
        get_input_line()
    }
}

fn main() {
    let code = &welcome();

    let exec_time = benchmark!{{ interpreter::test(code); }};
    println!("Time taken {} ms", exec_time);
}