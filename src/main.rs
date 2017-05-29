use std::env;
use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write, stdout};

#[macro_use]
mod benchmark;

mod lexer;
mod optimizer;

fn get_input_line() -> String {    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    String::from(input.trim())
}

/* fn get_input_char() -> char {
    std::io::stdin().chars().next().unwrap().ok().unwrap()
}

struct Brainfuck {
    code: String,
    code_pos: usize,
    jumps: Vec<usize>,
    tape: Vec<usize>,
    tape_pos: usize,

}

impl Brainfuck {
    fn new(code: String) -> Brainfuck {
        let jumps = make_jumptable(&code);
        Brainfuck {
            code: code,
            code_pos: 0,
            jumps: jumps,
            tape: vec![0; 30000],
            tape_pos: 0,

        }
    }
} */

fn read_file_as_string<P: AsRef<Path>>(path: P) -> String {
    let f = File::open(path);

    let mut input = String::new();
    f.expect("Error reading file.").read_to_string(&mut input).unwrap();
    String::from(input.trim())
}

fn welcome() -> String {
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

fn make_jumptable(code: &String) -> Vec<usize> {
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

fn clamp(val: usize, min_val: usize, max_val: usize) -> usize {
    cmp::max(cmp::min(max_val, val), min_val)
}

#[inline]
fn interpret_loop(code: &String) {
    let jumps = make_jumptable(&code);

    let mut code_pos = 0;
    
    const SIZE: usize = 30000;
    let mut tape = vec![0u8; 30000];
    let mut tape_pos = 0;

    let mut in_buffer = String::from("");

    while code_pos < code.len() {
        
        if let Some(c) = code.chars().nth(code_pos) {
            match c {
                // Basic Commands
                '+' => tape[tape_pos] = tape[tape_pos].wrapping_add(1),
                '-' => tape[tape_pos] = tape[tape_pos].wrapping_sub(1),
                '>' => tape_pos = clamp(tape_pos, 0, SIZE - 1) + 1,
                '<' => tape_pos = clamp(tape_pos, 1, SIZE) - 1,

                // Input/Output
                ',' => {
                    if in_buffer.len() <= 0 {
                        println!();
                        in_buffer += &get_input_line();
                    };
                    if in_buffer.len() > 0 {
                        tape[tape_pos] = match in_buffer.chars().nth(0) {
                            Some(char) => {
                                in_buffer.remove(0);
                                char
                            },
                            None => panic!("Exited by user"),
                        } as u8;
                    }
                },
                '.' => {
                    print!("{}", tape[tape_pos as usize] as char);
                    stdout().flush().expect("Failed to write buffered output to stdout");
                },

                // Loop/Branch Commands
                '[' => if tape[tape_pos] == 0 {
                    code_pos = jumps[code_pos];
                },
                ']' => if tape[tape_pos] != 0 {
                    code_pos = jumps[code_pos];
                },
                _ => {},

            }
        }
        code_pos += 1;
    }
    print!("\n");

}

fn main() {
    let code = strip_chars(&welcome());

    let exec_time = benchmark!{{ interpret_loop(&code); }};
    println!("Time taken {} ms", exec_time);
}