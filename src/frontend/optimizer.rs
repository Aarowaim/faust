use frontend::FaustCmd;
use frontend::BasicCmd;

pub fn full_optimize<T>(cmds: T) -> Vec<FaustCmd>
where T: Into<Vec<FaustCmd>> {
	let mut temp = cmds.into();
	temp = trim_right(&temp);
	temp = condense(&temp);
	temp = simple_clears(&temp);

	temp
}

fn trim_right(cmds: &Vec<FaustCmd>) -> Vec<FaustCmd> {
	let mut rp = cmds.len();

	while 0 < rp && ![FaustCmd::Output, FaustCmd::JumpNotZero]
		.contains(&cmds[rp - 1]) {
		rp -= 1;
	}
	cmds[..rp].to_vec()
}

fn condense(cmds: &Vec<FaustCmd>) -> Vec<FaustCmd> {
	let mut temp = Vec::new();

	let mut lp = 0;

	while lp < cmds.len() {
		let mut accumulator = 0;
		let current = cmds[lp].clone();

		match current {
			FaustCmd::Repeatable(initial_cmd, _) => {	

				while lp < cmds.len() {
					match cmds[lp].clone() {
						FaustCmd::Repeatable(current_cmd, cur_a) => {
							if current_cmd == initial_cmd {
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
				temp.push(FaustCmd::Repeatable(initial_cmd, accumulator));
			},
			_ => temp.push(current),
		}
		lp += 1;
		
	}
	temp
}

fn simple_clears(cmds: &Vec<FaustCmd>) -> Vec<FaustCmd> {
	let mut temp = Vec::new();
	let mut accumulator = Vec::new();

	let mut lp = 0;
	while lp < cmds.len() {
		accumulator.push(cmds[lp].clone());

		if accumulator.len() == 3 {

			if let FaustCmd::Repeatable(cmd, _) = accumulator[1].clone() {
				if accumulator[0] == FaustCmd::JumpEqualZero &&
				cmd == BasicCmd::Sub &&
				accumulator[2] == FaustCmd::JumpNotZero {
					println!("{}, {}, {}", accumulator[0], accumulator[1], accumulator[2]);
					accumulator = Vec::new();
					temp.push(FaustCmd::Clear);
				} else {
					temp.push(accumulator.remove(0));
				}
			} else {
				temp.push(accumulator.remove(0));
			}
		} else {
			temp.push(accumulator.remove(0));
		}
		lp += 1;
	}
	temp
}