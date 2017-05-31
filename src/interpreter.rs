use frontend::*;
use std::cmp;
use std::io::{self, Write, stdout};

const SIZE: usize = 30000;

struct Brainfuck {
    code: Vec<FaustCmd>,
    code_pos: usize,
    jumps: Vec<usize>,
    tape: Vec<u8>,
    tape_pos: usize,

}

fn get_input_line() -> String {    
    let mut input = String::new();
    println!();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    String::from(input.trim())
}

fn clamp(val: isize, min_val: usize, max_val: usize) -> usize {
    cmp::max(cmp::min(max_val as isize - 1, val), min_val as isize) as usize
}

impl Brainfuck {

    fn new(code: Vec<FaustCmd>) -> Brainfuck {
        let jumps = Brainfuck::make_jumptable(&code);
        Brainfuck {
            code: code,
            code_pos: 0,
            jumps: jumps,
            tape: vec![0; SIZE],
            tape_pos: 0,

        }
    }

    #[inline]
    fn interpret(&mut self) {

        let mut in_buffer = String::from("");

        while self.code_pos < self.code.len() {
            
            if let Some(c) = self.code.iter().nth(self.code_pos) {
                match c.clone() {
                    // Basic Commands
                    FaustCmd::Repeatable(cmd, n) 
                        => match cmd.clone() {
                            BasicCmd::Add => self.tape[self.tape_pos] = (self.tape[self.tape_pos] as u8).wrapping_add(n as u8),
                            BasicCmd::Sub => self.tape[self.tape_pos] = (self.tape[self.tape_pos] as u8).wrapping_sub(n as u8),
                            BasicCmd::Skip => self.tape_pos = clamp(self.tape_pos as isize + n as isize, 0, SIZE),
                            BasicCmd::Rewind => self.tape_pos = clamp(self.tape_pos as isize - n as isize, 0, SIZE),
                        },
                    // Input/Output
                    FaustCmd::Input => {
                        if in_buffer.len() <= 0 {
                            in_buffer += &get_input_line();
                        };
                        if in_buffer.len() > 0 {
                            
                            if let Some(chr) = in_buffer.chars().nth(0) {
                                in_buffer.remove(0);
                                self.tape[self.tape_pos] = chr as u8;
                            } else {
                                panic!("Exited by user");
                            }
                        };
                    },
                    FaustCmd::Output => {
                        print!("{}", self.tape[self.tape_pos] as u8 as char);
                        stdout().flush().expect("Failed to write buffered output to stdout");
                    },

                    // Loop/Branch Commands
                    FaustCmd::JumpEqualZero => if self.tape[self.tape_pos] == 0 {
                        self.code_pos = self.jumps[self.code_pos];
                    },
                    FaustCmd::JumpNotZero => if self.tape[self.tape_pos] != 0 {
                        self.code_pos = self.jumps[self.code_pos];
                    },
                    _ => {},

                };
            }
            self.code_pos += 1;
        }
        print!("\n");

    }

    fn make_jumptable(code: &Vec<FaustCmd>) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut jumps = vec![0usize; code.len()];

        for (i, c) in code.iter().enumerate() {
            match *c {
                FaustCmd::JumpEqualZero => stack.push(i),
                FaustCmd::JumpNotZero => if let Some(left) = stack.pop() {
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

}

pub fn test(code: &String) {
    let mut bf = Brainfuck::new(brainfuck::vanilla_brainfuck(code));
    bf.interpret();
    
}