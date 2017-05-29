use lexer::*;

impl Lexer for TokenLegend {

	fn lex(&self, code: &String) -> Vec<Token> {
		let mut tokens = Vec::new();

		for c in code.chars() {
			if let Some(&t) = self.map.get(&c.to_string()) {
				tokens.push(t.clone());
			}
		}
		tokens
	}

}

