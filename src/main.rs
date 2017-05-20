use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write, stdout};

use std::time::Instant;

fn get_input_line() -> String {    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    String::from(input.trim())
}

/* fn get_input_char() -> char {
    std::io::stdin().lock().chars().next()
} */

struct Brainfuck {
    code: String,
    code_pos: usize,
    jumps: Vec<usize>,
    tape: Vec<usize>,
    tape_pos: usize,

}

impl Brainfuck {
    fn new(code: String) -> Brainfuck {
        let jumps = preprocess(&code);
        Brainfuck {
            code: code,
            code_pos: 0,
            jumps: jumps,
            tape: vec![0; 30000],
            tape_pos: 0,

        }
    }
}

fn read_file_as_string<P: AsRef<Path>>(path: P) -> String {
    let f = File::open(path);

    let mut input = String::new();
    f.expect("Error reading file.").read_to_string(&mut input).unwrap();
    String::from(input.trim())
}

fn intro() -> String {
    println!("Brainfuck interpreter");

    let args: Vec<_> = env::args().collect();
    
    if args.len() > 1 {
        println!("Reading from file.");
    	read_file_as_string(&args[1])
    } else {
    	println!("Trying input");
    	get_input_line()
    }
}

fn preprocess(code: &String) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut jumps = vec![0usize; code.len()];

    for (i, c) in code.chars().enumerate() {
        match c {
            '[' => stack.push(i),
            ']' => if let Some(left) = stack.pop() {
                let right = i;
                jumps[left] = right;
                jumps[right] = left;
            },
            _ => continue,
        }
    }

    if stack.len() != 0 {
            panic!("Not enough left brackets in brainfuck code!")
    }

    jumps
}

fn strip_chars(code: &String) -> String {
    let mut s: String = String::from("");
    let keepers = ['+', '-', '<', '>', '[', ']', '.', ',', '#', '!'];

    for c in code.chars() {
        if keepers.contains(&c) {
            s.push(c);
        }
    }
    s
}

fn interpret_loop(code: String) {
    let jumps = preprocess(&code);
    let initial_t = Instant::now();
    let mut accumulator: u64 = 0;

    let mut code_pos = 0;
    
    let mut tape = vec![0u8; 30000];
    let mut tape_pos = 0;

    println!("code.len: {}", code.len());
    while code_pos < code.len() {
        // println!("position:\t{}\ncharacter:\t{}", code_pos, code.chars().nth(code_pos).unwrap());
        match code.chars().nth(code_pos) {
            // Basic Commands
            Some('+') => tape[tape_pos] = tape[tape_pos].wrapping_add(1),
            Some('-') => tape[tape_pos] = tape[tape_pos].wrapping_sub(1),
            Some('>') => tape_pos = tape_pos.wrapping_add(1),
            Some('<') => tape_pos = tape_pos.wrapping_sub(1),

            // Input/Output
            Some(',') => {},
            Some('.') => {
                print!("{}", tape[tape_pos as usize] as char);
                stdout().flush().expect("Failed to write buffered output to stdout");
            },

            // Loop/Branch Commands
            Some('[') => if tape[tape_pos] == 0 {
                code_pos = jumps[code_pos];
            },
            Some(']') => if tape[tape_pos] != 0 {
                code_pos = jumps[code_pos];
            },
            _ => {},
        }
        code_pos += 1;
        accumulator += 1;
    }
    let delta_t = initial_t.elapsed();
    let ms_elapsed = (delta_t.as_secs() * 1_000) + (delta_t.subsec_nanos() / 1_000_000) as u64;
    println!("{} instructions executed.\n{} instructions per millisecond", accumulator, accumulator / ms_elapsed)
    
}

fn main() {
    let code = strip_chars(&intro());
	interpret_loop(code);
}