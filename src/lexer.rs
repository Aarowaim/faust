use std::collections::HashMap;

enum BfCommand {
	Forward,		// >
	Back,			// <
	Increment,		// +
	Decrement,		// -

	OutputChar,		// .
	InputChar,		// ,
	SkipZero,		// [
	RewindNotZero,	// ]

	DebugPrint,		// # - print a few cells
	ToggleBufferIn,	// ! - Buffer characters to input for ,

	DebugBreak,		// # - Debugger Breakpoint
}

struct BfSpecification {
	map: HashMap<String, BfCommand>,
}

impl BfSpecification {

	fn lex(&self, s: &String) -> Vec<BfCommand> {
		let tokens: Vec<BfCommand> = Vec::new();
		for c in s.chars() {

		}
		Vec::new()
	}

}

