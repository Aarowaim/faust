use frontend::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Token {
	// Standard Instructions
	Forward,			// >
	Back,				// <
	Add,				// +
	Sub,				// -

	Output,				// .
	Input,				// ,
	SkipZero,			// [
	RewindNotZero,		// ]

	More(SpecialToken),

}

#[derive(Clone, PartialEq)]
enum SpecialToken {

	// Developer Instructions
	DebugPrint,			// # - print a few cells
	ToggleStream,		// ! - Buffer characters to input for ,

	Breakpoint,			// # - Debugger Breakpoint
}

impl Frontend for HashMap<String, Token> {

	fn to_faust(&self, code: &String) -> Vec<FaustInstruction> {
		let mut tokens = Vec::new();

		for c in code.chars() {
			if let Some(t) = self.get(&c.to_string()) {
				tokens.push(t.clone());
			}
		}
		compile(tokens)
	}

}

fn condense(t: Token, n: usize) -> Option<FaustInstruction> {
	match t {
		Token::Add => {
			println!("<Add> {}", n);
			Some(FaustInstruction::Add(n))
		},
		Token::Sub => {
			println!("<Sub> {}", n);
			Some(FaustInstruction::Sub(n))
		},
		Token::Forward => {
			println!("<Skip> {}", n);
			Some(FaustInstruction::Skip(n))
		},
		Token::Back => {
			println!("<Rewind> {}", n);
			Some(FaustInstruction::Rewind(n))
		},
		_ => {
			None
		},
	}
}

fn lookup(t: Token) -> Option<FaustInstruction> {
	match t {
		Token::Add => {
			Some(FaustInstruction::Add(1))
		},
		Token::Sub => {
			Some(FaustInstruction::Sub(1))
		},
		Token::Forward => {
			Some(FaustInstruction::Skip(1))
		},
		Token::Back => {
			Some(FaustInstruction::Rewind(1))
		},
		_ => {
			None
		},
	}
}

fn compile(tokens: Vec<Token>) -> Vec<FaustInstruction> {
	let mut instructions = Vec::new();
	let mut i = 0;

	while i < tokens.len() {
		let mut accumulator = 0;
		let current = tokens[i].clone();

		match current {
			Token::Add | Token::Sub |
			Token::Forward | Token::Back => {	

				while i < tokens.len() && tokens[i] == current {
					accumulator += 1;
					i += 1;
				} 	i -= 1; // outer loop already performs one increment

				if let Some(instruction) = condense(current, accumulator) {
					instructions.push(instruction);
				}
			},
			_ => {
				// Handle bracket simplification without recursion
				// The brainfuck programs worth interpreting will
				// cause overflows if nested calls occur
			},
		}
		i += 1;
	}

	instructions
}

fn vanilla_brainfuck(code: &String) -> Vec<FaustInstruction> {
	let mut m = HashMap::new();

	m.insert('>'.to_string(), Token::Forward);
	m.insert('<'.to_string(), Token::Back);
	m.insert('+'.to_string(), Token::Add);
	m.insert('-'.to_string(), Token::Sub);

	m.insert('.'.to_string(), Token::Output);
	m.insert(','.to_string(), Token::Input);
	m.insert('['.to_string(), Token::SkipZero);
	m.insert(']'.to_string(), Token::RewindNotZero);

	m.to_faust(code)
}

pub fn test(code: &String) {
	vanilla_brainfuck(code);
}