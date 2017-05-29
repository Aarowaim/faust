use std::collections::HashMap;

mod brainfuck;

#[derive(Clone)]
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

#[derive(Clone)]
enum SpecialToken {
	// Syntactic Elements
	Value(usize),
	Ident(String),
	Block(Vec<Token>),

	// Developer Instructions
	DebugPrint,			// # - print a few cells
	ToggleStream,		// ! - Buffer characters to input for ,

	Breakpoint,			// # - Debugger Breakpoint
}

struct TokenLegend {
	map: HashMap<String, Token>,
}

trait Lexer {
	fn lex(&self, code: &String) -> Vec<Token>;
}
