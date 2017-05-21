use ast::*;
use buffered_iterator::*;
use scanning::*;

struct Parser<'a> {
	scanner: BufferedIterator<Token, Scanner<'a>>,
}

impl<'a> Parser<'a> {
	fn new(input: &String) -> Parser {
		let scanner = Scanner::new(input);
		let buf = BufferedIterator::new(scanner);

		Parser {
			scanner: buf
		}
	}

	fn get_token(&mut self) -> Option<Token> {
		return self.scanner.pop()
	}

	fn put_token(&mut self, t: Token) {
		self.scanner.push(t)
	}

	fn expect_token(&mut self) -> Option<Token> {
		return match self.get_token() {
			Some(t) => Some(t),
			None => {
				println!("Unexpected end of input");
				None
			}
		}
	}

	fn parse_ast(&mut self) -> Option<Ast> {
		trace!("parse_ast");

		if let Some(cmd) = self.parse_command() {
			return Some(Ast::Command(cmd));
		}

		if let Some(expr) = self.parse_expression() {
			return Some(Ast::Expression(expr));
		}

		return None;
	}

	fn parse_command(&mut self) -> Option<Command> {
		trace!("parse_command");

		let t = unwrap!(self.get_token(), {
			return None;
		});

		if let Token { token_type: TokenType::Identifier { ref str, .. }, .. } = t {
			match str.as_str() {
				"exit" | "quit" => return Some(Command::Exit),
				_ => {},
			}
		}

		self.put_token(t);

		return None
	} // parse_command

	fn parse_expression(&mut self) -> Option<Expression> {
		trace!("parse_expression");

		return self.parse_bitor();
	}

	fn parse_bitor(&mut self) -> Option<Expression> {
		trace!("parse_bitor");

		let mut expr = unwrap!(self.parse_bitxor(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::Pipe => op = BinaryOp::BitOr,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_bitxor(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	} // parse_bitor

	fn parse_bitxor(&mut self) -> Option<Expression> {
		trace!("parse_bitxor");

		let mut expr = unwrap!(self.parse_bitand(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::Caret => op = BinaryOp::BitXor,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_bitand(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	} // parse_bitxor

	fn parse_bitand(&mut self) -> Option<Expression> {
		trace!("parse_bitand");

		let mut expr = unwrap!(self.parse_shift(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::Ampersand => op = BinaryOp::BitAnd,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_shift(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	} // parse_bitand

	fn parse_shift(&mut self) -> Option<Expression> {
		trace!("parse_shift");

		let mut expr = unwrap!(self.parse_add_subtract(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::LeftAngleBracketX2 => op = BinaryOp::LeftShift,
				TokenType::RightAngleBracketX2 => op = BinaryOp::RightShift,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_add_subtract(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	} // parse_shift

	fn parse_add_subtract(&mut self) -> Option<Expression> {
		trace!("parse_add_subtract");

		let mut expr = unwrap!(self.parse_multiply_divide(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::Plus => op = BinaryOp::Plus,
				TokenType::Minus => op = BinaryOp::Minus,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_multiply_divide(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	}

	fn parse_multiply_divide(&mut self) -> Option<Expression> {
		trace!("parse_multiply_divide");

		let mut expr = unwrap!(self.parse_exponent(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::Star => op = BinaryOp::Multiply,
				TokenType::ForwardSlash => op = BinaryOp::Divide,
				TokenType::Percent => op = BinaryOp::Modulo,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_exponent(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	} // parse_multiply_divide

	fn parse_exponent(&mut self) -> Option<Expression> {
		trace!("parse_exponent");

		let mut expr = unwrap!(self.parse_unary(), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op;
			match t.token_type {
				TokenType::StarX2 => op = BinaryOp::Exponent,
				_ => {
					self.put_token(t);
					break;
				}
			}

			let right = unwrap!(self.parse_unary(), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		return Some(expr);
	} // parse_exponent

	fn parse_unary(&mut self) -> Option<Expression> {
		trace!("parse_unary");

		let t = unwrap!(self.expect_token(), {
			return None;
		});

		let op;
		match t.token_type {
			TokenType::Minus => op = UnaryOp::Negate,
			TokenType::Bang => op = UnaryOp::Not,
			_ => {
				self.put_token(t);
				return self.parse_primary();
			}
		}

		return match self.parse_unary() {
			Some(right) => Some(Expression::Unary(Unary {
				op: op,
				right: Box::new(right)
			})),
			None => None,
		};
	} // parse_unary

	fn parse_primary(&mut self) -> Option<Expression> {
		trace!("parse_primary");

		let t = unwrap!(self.expect_token(), {
			return None;
		});

		return match t.token_type {
			TokenType::Number { str } => {
				match str.parse::<f64>() {
					Ok(f) => Some(Expression::Literal(Literal::Number(f))),
					Err(msg) => {
						println!("{}", msg);
						None
					}
				}
			},
			TokenType::Identifier { str } => {
				Some(Expression::Variable(Variable { name: str }))
			},
			TokenType::LeftParen => {
				let expr = self.parse_expression();
				match self.expect_token() {
					Some(Token { token_type: TokenType::RightParen, .. }) => expr,
					Some(t) => {
						println!("Expected token RightParen, but found '{:?}' (line {}, col {})",
							t.token_type, t.line, t.column);
						None
					},
					None => None,
				}
			},
			_ => {
				println!("Unexpected token '{:?}' (line {}, col {})",
					t.token_type, t.line, t.column);
				None
			}
		} // match
	} // parse_primary
} // Parser

pub fn parse<'a>(input: &'a String) -> Option<Ast> {
	Parser::new(input).parse_ast()
}
