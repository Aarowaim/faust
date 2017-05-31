pub mod brainfuck;
mod optimizer;

use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum BasicCmd {
	Skip,
	Rewind,
	Add,
	Sub,
}

impl Display for BasicCmd {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

// workaround because FaustCmd::Repeatable is 'not a valid type'
#[derive(Clone, PartialEq, Debug)]
pub struct Repeatable(BasicCmd, usize);

impl Display for Repeatable {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum FaustCmd {
	Repeatable(BasicCmd, usize),
	Addressed(Repeatable, usize),
	Clear,

	Output,
	Input,
	JumpEqualZero,
	JumpNotZero,

	DebugPrint,
	Breakpoint,

	ToggleBuffer,
	Buffer(String),

	// Iteration primitives
	For(Vec<FaustCmd>, usize), // loop and reduce by usize
	ScanFwd(Vec<FaustCmd>, usize), // do something and skip by usize
	ScanBk(Vec<FaustCmd>, usize), // do something and rewind by usize
}

impl Display for FaustCmd {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<{:?}>", self)
    }
}

trait Frontend {

	fn basic(&self, code: &String) -> Vec<FaustCmd>;

	fn optimize(&self, code: &String) -> Vec<FaustCmd> {
		optimizer::full_optimize(self.basic(code))
	}
}
