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
	let mut commands = Vec::new();

	for t in tokens {
		let cmd = match t {
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
		commands.push(cmd);
	}
	commands
}

pub fn vanilla_brainfuck(code: &String) -> Vec<FaustCmd> {
	let mut m = HashMap::new();

	let pairs = vec!{
		('>', Token::Forward), 
		('<', Token::Back),
		('+', Token::Add),
		('-', Token::Sub),
		('.', Token::Output),
		('[', Token::JumpEqualZero),
		(']', Token::JumpNotZero),
		('!', Token::ToggleBuffer)};

	for p in pairs {
		let (symbol, token) = p;
		m.insert(symbol.to_string(), token);
	};


	let instructions = m.optimize(code);

	instructions
}