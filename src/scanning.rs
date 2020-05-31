use crate::buffered_iterator::*;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
	Ampersand,
	AmpersandEqual,
	Bang,
	Caret,
	CaretEqual,
	Equal,
	ForwardSlash,
	ForwardSlashEqual,
	Identifier { str: String },
	LeftAngleBracketX2,
	LeftAngleBracketX2Equal,
	LeftParen,
	Minus,
	MinusEqual,
	NewLine,
	Number { str: String, prefix: String },
	Percent,
	PercentEqual,
	Pipe,
	PipeEqual,
	Plus,
	PlusEqual,
	RightAngleBracketX2,
	RightAngleBracketX2Equal,
	RightParen,
	Star,
	StarEqual,
	StarX2,
	StarX2Equal,
}

pub struct Token {
	pub token_type: TokenType,
	pub line: u32,
	pub column: u32,
	pub length: u32,
}

pub struct Scanner<'a> {
	chars: BufferedIterator<char, Chars<'a>>,
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
		let chars = input.chars();
		let buf = BufferedIterator::new(chars);

		Scanner {
			chars: buf,
			line: 1,
			column: 1,
		}
	}

	fn expected_char(&self, expected: char, found: char) {
		println!(
			"Expected '{}' but found '{}' instead (line {}, column {})",
			expected, found, self.line, self.column
		);
	}

	fn unexpected_char(&self, found: char) {
		println!(
			"Unexpected character '{}' (line {}, column {})",
			found, self.line, self.column
		);
	}

	fn unexpected_end_of_input(&self) {
		println!(
			"Unexpected end of input (line {}, column {})",
			self.line, self.column
		);
	}

	fn get_char(&mut self) -> Option<char> {
		self.column += 1;
		self.chars.pop()
	}

	fn put_char(&mut self, c: char) {
		self.column -= 1;
		self.chars.push(c)
	}

	fn expect_char(&mut self, expected: char) -> bool {
		match self.get_char() {
			Some(c) if c == expected => true,
			Some(c) => {
				self.expected_char(expected, c);
				false
			}
			None => {
				self.unexpected_end_of_input();
				false
			}
		}
	}

	fn consume_char(&mut self, char: char) -> bool {
		match self.get_char() {
			Some(c) if c == char => true,
			Some(c) => {
				self.put_char(c);
				false
			}
			None => false,
		}
	}

	fn consume_char_of<FPredicate>(&mut self, check_char: FPredicate) -> Option<char>
	where
		FPredicate: Fn(char) -> bool,
	{
		match self.get_char() {
			Some(c) if check_char(c) => Some(c),
			Some(c) => {
				self.put_char(c);
				None
			}
			None => None,
		}
	}

	fn new_token(&self, token_type: TokenType, length: u32) -> Option<Token> {
		let t = Token {
			token_type,
			line: self.line,
			column: self.column - length,
			length,
		};

		trace!(
			"{:?} @{}:{}, len {}",
			t.token_type,
			t.line,
			t.column,
			t.length
		);

		Some(t)
	}

	fn next(&mut self) -> Option<Token> {
		let mut c;

		// find the next non-whitespace character
		loop {
			c = unwrap!(self.get_char(), {
				return None;
			});

			// keep new lines, even though they are "whitespace"
			if c == '\n' || !c.is_whitespace() {
				break;
			}
		}

		match c {
			'(' => self.new_token(TokenType::LeftParen, 1),
			')' => self.new_token(TokenType::RightParen, 1),
			'+' => self.scan_plus(),
			'-' => self.scan_minus(),
			'*' => self.scan_star(),
			'^' => self.scan_caret(),
			'%' => self.scan_percent(),
			'/' => self.scan_forward_slash(),
			'!' => self.new_token(TokenType::Bang, 1),
			'|' => self.scan_pipe(),
			'&' => self.scan_ampersand(),
			'<' => self.scan_left_angle_bracket(),
			'>' => self.scan_right_angle_bracket(),
			'=' => self.new_token(TokenType::Equal, 1),
			'\n' => self.scan_new_line(),
			'_' => {
				self.put_char(c);
				self.scan_identifier()
			}
			'.' | '0'..='9' => {
				self.put_char(c);
				self.scan_number()
			}
			_ => {
				self.put_char(c);
				if c.is_alphabetic() {
					self.scan_identifier()
				} else {
					self.unexpected_char(c);
					None
				}
			}
		}
	}

	fn scan_plus(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::PlusEqual, 2)
		} else {
			self.new_token(TokenType::Plus, 1)
		}
	}

	fn scan_minus(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::MinusEqual, 2)
		} else {
			self.new_token(TokenType::Minus, 1)
		}
	}

	fn scan_star(&mut self) -> Option<Token> {
		if self.consume_char('*') {
			if self.consume_char('=') {
				self.new_token(TokenType::StarX2Equal, 3)
			} else {
				self.new_token(TokenType::StarX2, 2)
			}
		} else if self.consume_char('=') {
			self.new_token(TokenType::StarEqual, 2)
		} else {
			self.new_token(TokenType::Star, 1)
		}
	}

	fn scan_caret(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::CaretEqual, 2)
		} else {
			self.new_token(TokenType::Caret, 1)
		}
	}

	fn scan_percent(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::PercentEqual, 2)
		} else {
			self.new_token(TokenType::Percent, 1)
		}
	}

	fn scan_forward_slash(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::ForwardSlashEqual, 2)
		} else {
			self.new_token(TokenType::ForwardSlash, 1)
		}
	}

	fn scan_pipe(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::PipeEqual, 2)
		} else {
			self.new_token(TokenType::Pipe, 1)
		}
	}

	fn scan_ampersand(&mut self) -> Option<Token> {
		if self.consume_char('=') {
			self.new_token(TokenType::AmpersandEqual, 2)
		} else {
			self.new_token(TokenType::Ampersand, 1)
		}
	}

	fn scan_left_angle_bracket(&mut self) -> Option<Token> {
		if self.expect_char('<') {
			if self.consume_char('=') {
				self.new_token(TokenType::LeftAngleBracketX2Equal, 3)
			} else {
				self.new_token(TokenType::LeftAngleBracketX2, 2)
			}
		} else {
			None
		}
	}

	fn scan_right_angle_bracket(&mut self) -> Option<Token> {
		if self.expect_char('>') {
			if self.consume_char('=') {
				self.new_token(TokenType::RightAngleBracketX2Equal, 3)
			} else {
				self.new_token(TokenType::RightAngleBracketX2, 2)
			}
		} else {
			None
		}
	}

	fn scan_new_line(&mut self) -> Option<Token> {
		let t = self.new_token(TokenType::NewLine, 1);
		self.line += 1;
		self.column = 0;
		t
	} // scan_new_line

	fn scan_number(&mut self) -> Option<Token> {
		let bin = |c: char| matches!(c, '_' | '0'..='1');
		let oct = |c: char| matches!(c, '_' | '0'..='7');
		let dec = |c: char| matches!(c, '_' | '0'..='9');
		let fdec = |c: char| matches!(c, '_' | '0'..='9' | '.');
		let hex = |c: char| matches!(c, '_' | '0'..='9' | 'a'..='f' | 'A'..='F');

		let start = self.column;
		let mut pred: &dyn Fn(char) -> bool = &fdec;
		let mut value = String::new();
		let mut prefix = String::new();

		// check for binary/octal/hexadecimal literal prefixes
		if self.consume_char('0') {
			match self.get_char() {
				Some('b') => {
					prefix.push_str("0b");
					pred = &bin;
				}
				Some('d') => {
					prefix.push_str("0d");
					pred = &fdec;
				}
				Some('o') => {
					prefix.push_str("0o");
					pred = &oct;
				}
				Some('x') => {
					prefix.push_str("0x");
					pred = &hex;
				}
				Some(c1) => {
					self.put_char(c1);
					self.put_char('0');
				}
				None => self.put_char('0'),
			}
		}

		while let Some(c) = self.consume_char_of(pred) {
			if c != '_' {
				value.push(c);
			}

			if c == '.' {
				pred = &dec;
			}
		}

		self.new_token(
			TokenType::Number { str: value, prefix },
			self.column - start,
		)
	} // scan_number

	fn scan_identifier(&mut self) -> Option<Token> {
		let start = self.column;
		let mut str = String::new();
		str.push(self.get_char().unwrap());

		while let Some(c) = self.get_char() {
			match c {
				'_' => {}
				_ => {
					if !c.is_alphanumeric() {
						self.put_char(c);
						break;
					}
				}
			}

			str.push(c);
		}

		self.new_token(TokenType::Identifier { str }, self.column - start)
	} // scan_identifier
} // Scanner

#[cfg(test)]
mod tests {
	use crate::scanning::*;

	fn setup(input: &str) -> Scanner {
		Scanner::new(input)
	}

	fn expect(scanner: &mut Scanner, tt: TokenType) {
		let token = unwrap!(scanner.next(), {
			panic!("Expected Token {:?} but found None", tt);
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
		expect(
			&mut s,
			TokenType::Identifier {
				str: "ans".to_string(),
			},
		);
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
	fn scan_equal() {
		let mut s = setup("=");
		expect(&mut s, TokenType::Equal);
	}

	#[test]
	fn scan_new_line() {
		let mut s = setup("\n");
		expect(&mut s, TokenType::NewLine);
	}

	#[test]
	fn scan_number() {
		let mut s = setup("0b01 0o01234567 0x0123456789abcdefABCDEF 0123456789 11_11 11.11");
		expect(
			&mut s,
			TokenType::Number {
				str: "01".to_string(),
				prefix: "0b".to_string(),
			},
		);
		expect(
			&mut s,
			TokenType::Number {
				str: "01234567".to_string(),
				prefix: "0o".to_string(),
			},
		);
		expect(
			&mut s,
			TokenType::Number {
				str: "0123456789abcdefABCDEF".to_string(),
				prefix: "0x".to_string(),
			},
		);
		expect(
			&mut s,
			TokenType::Number {
				str: "0123456789".to_string(),
				prefix: "".to_string(),
			},
		);
		expect(
			&mut s,
			TokenType::Number {
				str: "1111".to_string(),
				prefix: "".to_string(),
			},
		);
		expect(
			&mut s,
			TokenType::Number {
				str: "11.11".to_string(),
				prefix: "".to_string(),
			},
		);
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
