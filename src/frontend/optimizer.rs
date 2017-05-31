use frontend::FaustCmd;

pub fn full_optimize<T>(instructions: T) -> Vec<FaustCmd>
where T: Into<Vec<FaustCmd>> {
	let mut temp = instructions.into();
	temp = trim_right(&temp);
	temp = condense(&temp);
	temp
}

fn trim_right(instructions: &Vec<FaustCmd>) -> Vec<FaustCmd> {
	let mut rp = instructions.len();

	while 0 < rp && ![FaustCmd::Output, FaustCmd::JumpNotZero]
		.contains(&instructions[rp - 1]) {
		rp -= 1;
	}
	instructions[..rp].to_vec()
}

fn condense(cmds: &Vec<FaustCmd>) -> Vec<FaustCmd> {
	let mut temp = Vec::new();

	let mut lp = 0;

	while lp < cmds.len() {
		let mut accumulator = 0;
		let current = cmds[lp].clone();

		match current {
			FaustCmd::Repeatable(initial_c, _) => {	

				while lp < cmds.len() {
					match cmds[lp].clone() {
						FaustCmd::Repeatable(cur_c, cur_a) => {
							if cur_c == initial_c {
								accumulator += cur_a;
								lp += 1;
							} else {
								break;
							}
						},
						_ => break,
					}
				}
				lp -= 1; // outer loop already performs one increment
				temp.push(FaustCmd::Repeatable(initial_c, accumulator));
			},
			_ => temp.push(current),
		}
		lp += 1;
		
	}
	temp
}