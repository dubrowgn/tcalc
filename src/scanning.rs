use std::str::Chars;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
	Ampersand,
	Bang,
	Caret,
	ForwardSlash,
	Identifier { str: String },
	LeftAngleBracketX2,
	LeftParen,
	Minus,
	NewLine,
	Number { str: String },
	Percent,
	Pipe,
	Plus,
	RightAngleBracketX2,
	RightParen,
	Star,
	StarX2,
}

pub struct Token {
	pub token_type: TokenType,
	pub line: u32,
	pub column: u32,
	pub length: u32
}

pub struct Scanner<'a> {
	chars: Chars<'a>,
	next_char: Option<char>,
	line: u32,
	column: u32,
}

impl<'a> Iterator for Scanner<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		self.next()
	}
}

impl<'a> Scanner<'a> {
	pub fn new(input: &'a str) -> Scanner<'a> {
		let mut chars = input.chars();
		Scanner {
			next_char: chars.next(),
			chars: chars,
			line: 1,
			column: 1,
		}
	}

	fn get_char(&mut self) -> Option<char> {
		let next = self.next_char;
		self.next_char = self.chars.next();
		self.column += 1;
		next
	}

	fn peek_char(&self) -> Option<char> {
		self.next_char
	}

	fn skip_char(&mut self) {
		self.next_char = self.chars.next();
		self.column += 1;
	}

	fn expect_char(&mut self, expected: char) -> Option<char> {
		match self.peek_char() {
			Some(c) => {
				if c == expected {
					Some(c)
				} else {
					println!("Expected '{}' but found '{}' instead (line {}, column {})",
						expected, c, self.line, self.column);
					None
				}
			},
			None => {
				println!("Expected '{}' but found end-of-input instead (line {}, column {})",
					expected, self.line, self.column);
				None
			},
		}
	}

	fn new_token(&self, token_type: TokenType, length: u32) -> Option<Token> {
		let t = Token {
			token_type: token_type,
			line: self.line,
			column: self.column - length,
			length: length
		};

		trace!("{:?} @{}:{}, len {}",
			t.token_type, t.line, t.column, t.length);

		Some(t)
	}

	fn next(&mut self) -> Option<Token> {
		let mut c;

		// find the next non-whitespace character
		loop {
			c = unwrap!(self.peek_char(), {
				return None;
			});

			// keep new lines, even though they are "whitespace"
			if c == '\n' || !c.is_whitespace() {
				break;
			}
			
			self.skip_char();
		}

		match c {
			'(' => self.scan_left_paren(),
			')' => self.scan_right_paren(),
			'+' => self.scan_plus(),
			'-' => self.scan_minus(),
			'*' => self.scan_star(),
			'^' => self.scan_caret(),
			'%' => self.scan_percent(),
			'/' => self.scan_forward_slash(),
			'!' => self.scan_bang(),
			'|' => self.scan_pipe(),
			'&' => self.scan_ampersand(),
			'<' => self.scan_left_angle_bracket(),
			'>' => self.scan_right_angle_bracket(),
			'\n' => self.scan_new_line(),
			'.' => self.scan_number(),
			_ => {
				if c.is_numeric() {
					self.scan_number()
				} else if c.is_alphabetic() {
					self.scan_identifier()
				} else {
					println!("unexpected character '{}' (line {}, column {})",
						c, self.line, self.column);
					Option::None
				}
			}
		}
	} // next

	fn scan_left_paren(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::LeftParen, 1)
	}

	fn scan_right_paren(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::RightParen, 1)
	}

	fn scan_plus(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Plus, 1)
	}

	fn scan_minus(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Minus, 1)
	}

	fn scan_star(&mut self) -> Option<Token> {
		self.skip_char();
		match self.peek_char() {
			Some('*') => {
				self.skip_char();
				self.new_token(TokenType::StarX2, 2)
			}
			_ => self.new_token(TokenType::Star, 1)
		}
	}

	fn scan_caret(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Caret, 1)
	}

	fn scan_percent(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Percent, 1)
	}

	fn scan_forward_slash(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::ForwardSlash, 1)
	}

	fn scan_bang(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Bang, 1)
	}

	fn scan_pipe(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Pipe, 1)
	}

	fn scan_ampersand(&mut self) -> Option<Token> {
		self.skip_char();
		self.new_token(TokenType::Ampersand, 1)
	}

	fn scan_left_angle_bracket(&mut self) -> Option<Token> {
		self.skip_char();
		match self.expect_char('<') {
			Some(_) => {
				self.skip_char();
				self.new_token(TokenType::LeftAngleBracketX2, 2)
			},
			None => None,
		}
	}

	fn scan_right_angle_bracket(&mut self) -> Option<Token> {
		self.skip_char();
		match self.expect_char('>') {
			Some(_) => {
				self.skip_char();
				self.new_token(TokenType::RightAngleBracketX2, 2)
			},
			None => None,
		}
	}

	fn scan_new_line(&mut self) -> Option<Token> {
		self.skip_char();
		let t = self.new_token(TokenType::NewLine, 1);
		self.line += 1;
		self.column = 0;
		t
	}

	fn scan_number(&mut self) -> Option<Token> {
		let start = self.column;
		let mut is_float = false;
		let mut str = String::new();

		while let Some(c) = self.peek_char() {
			match c {
				'.' => {
					if is_float {
						break;
					}
					is_float = true;
				},
				'_' => {
					self.skip_char();
					continue;
				},
				_ => if !c.is_numeric() {
					break;
				}
			}

			self.skip_char();
			str.push(c);
		}

		self.new_token(
			TokenType::Number{ str: str },
			self.column - start
		)
	} // scan_number

	fn scan_identifier(&mut self) -> Option<Token> {
		let start = self.column;

		let mut str = String::new();
		str.push(self.get_char().unwrap());

		while let Some(c) = self.peek_char() {
			match c {
				'_' => { },
				_ => if !c.is_alphanumeric() {
					break;
				}
			}

			self.skip_char();
			str.push(c);
		}

		self.new_token(
			TokenType::Identifier{ str: str },
			self.column - start
		)
	} // scan_identifier
} // Scanner

#[cfg(test)]
mod tests {
	use scanning::*;

	fn setup(input: &str) -> Scanner {
		Scanner::new(input)
	}

	fn expect(scanner: &mut Scanner, tt: TokenType) {
		let token = unwrap!(scanner.next(), {
			panic!("Expected token but found None");
		});

		assert_eq!(token.token_type, tt);
	}

	#[test]
	fn scan_ampersand() {
		let mut s = setup("&");
		expect(&mut s, TokenType::Ampersand);
	}

	#[test]
	fn scan_bang() {
		let mut s = setup("!");
		expect(&mut s, TokenType::Bang);
	}

	#[test]
	fn scan_caret() {
		let mut s = setup("^");
		expect(&mut s, TokenType::Caret);
	}

	#[test]
	fn scan_forward_slash() {
		let mut s = setup("/");
		expect(&mut s, TokenType::ForwardSlash);
	}

	#[test]
	fn scan_identifier() {
		let mut s = setup("ans");
		expect(&mut s, TokenType::Identifier { str: "ans".to_string() });
	}

	#[test]
	fn scan_left_angle_bracket_x2() {
		let mut s = setup("<<");
		expect(&mut s, TokenType::LeftAngleBracketX2);
	}

	#[test]
	fn scan_left_paren() {
		let mut s = setup("(");
		expect(&mut s, TokenType::LeftParen);
	}

	#[test]
	fn scan_minus() {
		let mut s = setup("-");
		expect(&mut s, TokenType::Minus);
	}

	#[test]
	fn scan_new_line() {
		let mut s = setup("\n");
		expect(&mut s, TokenType::NewLine);
	}

	#[test]
	fn scan_number() {
		let mut s = setup("123");
		expect(&mut s, TokenType::Number { str: "123".to_string() });
	}

	#[test]
	fn scan_percent() {
		let mut s = setup("%");
		expect(&mut s, TokenType::Percent);
	}

	#[test]
	fn scan_pipe() {
		let mut s = setup("|");
		expect(&mut s, TokenType::Pipe);
	}

	#[test]
	fn scan_plus() {
		let mut s = setup("+");
		expect(&mut s, TokenType::Plus);
	}

	#[test]
	fn scan_right_angle_bracket_x2() {
		let mut s = setup(">>");
		expect(&mut s, TokenType::RightAngleBracketX2);
	}

	#[test]
	fn scan_right_paren() {
		let mut s = setup(")");
		expect(&mut s, TokenType::RightParen);
	}

	#[test]
	fn scan_star() {
		let mut s = setup("*");
		expect(&mut s, TokenType::Star);
	}

	#[test]
	fn scan_star_x2() {
		let mut s = setup("**");
		expect(&mut s, TokenType::StarX2);
	}
} // mod tests
