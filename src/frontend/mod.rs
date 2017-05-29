pub mod brainfuck;

pub enum FaustInstruction {
	Skip(usize),
	Rewind(usize),

	Add(usize),
	Sub(usize),
	Clear,

	Output,
	Input,

	// Iteration primitives
	For(Vec<FaustInstruction>, usize), // loop and reduce by usize
	ScanFwd(Vec<FaustInstruction>, usize), // do something and skip by usize
	ScanBk(Vec<FaustInstruction>, usize), // do something and rewind by usize

}

trait Frontend {
	fn to_faust(&self, code: &String) -> Vec<FaustInstruction>;
}
