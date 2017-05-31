use frontend::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Debug)]
enum Token {
	// Standard Instructions
	Forward,			// >
	Back,				// <
	Add,				// +
	Sub,				// -

	Output,				// .
	Input,				// ,
	JumpEqualZero,		// [
	JumpNotZero,		// ]

	// Developer Instructions
	DebugPrint,			// # - Print a few cells
	Breakpoint,			// # - Debugger Breakpoint

	ToggleBuffer,		// ! - Buffer characters to input for ,
	Buffer(String),
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<{:?}>", self)
    }
}

impl Frontend for HashMap<String, Token> {

	fn basic(&self, code: &String) -> Vec<FaustCmd> {
		let mut tokens = Vec::new();
		let mut is_buffering = false;
		let mut buf = String::new();

		for c in code.chars() {

			if let Some(t) = self.get(&c.to_string()) {

				if t == &Token::ToggleBuffer {

					if is_buffering {

						tokens.push(Token::Buffer(buf));
						buf = String::new();
					}
					is_buffering = !is_buffering;
				}
				tokens.push(t.clone());

			} else if is_buffering {

				buf += &c.to_string();
			}
		}
		translate(tokens)
	}
}

fn translate(tokens: Vec<Token>) -> Vec<FaustCmd> {
	let mut instructions = Vec::new();

	for t in tokens {
		let inst = match t {
			Token::Forward			=> FaustCmd::Repeatable(BasicCmd::Skip, 1),
			Token::Back				=> FaustCmd::Repeatable(BasicCmd::Rewind, 1),
			Token::Add 				=> FaustCmd::Repeatable(BasicCmd::Add, 1),
			Token::Sub				=> FaustCmd::Repeatable(BasicCmd::Sub, 1),

			Token::Output 			=> FaustCmd::Output,
			Token::Input 			=> FaustCmd::Input,
			Token::JumpEqualZero 	=> FaustCmd::JumpEqualZero,
			Token::JumpNotZero 		=> FaustCmd::JumpNotZero,

			Token::DebugPrint		=> FaustCmd::DebugPrint,
			Token::Breakpoint		=> FaustCmd::Breakpoint,

			Token::ToggleBuffer		=> FaustCmd::ToggleBuffer,
			Token::Buffer(s)		=> FaustCmd::Buffer(s),

		};
		instructions.push(inst);
	}
	instructions
}

pub fn vanilla_brainfuck(code: &String) -> Vec<FaustCmd> {
	let mut m = HashMap::new();

	m.insert('>'.to_string(), Token::Forward);
	m.insert('<'.to_string(), Token::Back);
	m.insert('+'.to_string(), Token::Add);
	m.insert('-'.to_string(), Token::Sub);

	m.insert('.'.to_string(), Token::Output);
	m.insert(','.to_string(), Token::Input);
	m.insert('['.to_string(), Token::JumpEqualZero);
	m.insert(']'.to_string(), Token::JumpNotZero);
	m.insert('!'.to_string(), Token::ToggleBuffer);

	let instructions = m.optimize(code);

	instructions
}