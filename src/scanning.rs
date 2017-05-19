use std::str::Chars;

#[derive(Debug)]
pub enum TokenType {
	LeftParen,
	RightParen,
	Plus,
	Minus,
	Star,
	Caret,
	Percent,
	StarStar,
	ForwardSlash,
	Number { str: String },
	Identifier { str: String },
	NewLine,
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
	pub fn new(input: &'a String) -> Scanner<'a> {
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

			if !c.is_whitespace() {
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
				self.new_token(TokenType::StarStar, 2)
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
