use ast::*;
use buffered_iterator::*;
use scanning::*;

struct Parser<'a> {
	scanner: BufferedIterator<Token, Scanner<'a>>,
}

impl<'a> Parser<'a> {
	fn new(input: &str) -> Parser {
		let scanner = Scanner::new(input);
		let buf = BufferedIterator::new(scanner);

		Parser {
			scanner: buf
		}
	}

	fn get_token(&mut self) -> Option<Token> {
		self.scanner.pop()
	}

	fn put_token(&mut self, t: Token) {
		self.scanner.push(t)
	}

	fn expect_token(&mut self) -> Option<Token> {
		match self.get_token() {
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

		None
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

		None
	} // parse_command

	fn parse_expression(&mut self) -> Option<Expression> {
		trace!("parse_expression");

		self.parse_bitor()
	}

	fn parse_binary<FOp, FNext>( &mut self, map_op: FOp, parse_next: FNext) -> Option<Expression>
		where FOp: Fn(&TokenType) -> Option<BinaryOp>, FNext: Fn(&mut Parser) -> Option<Expression>
	{
		let mut expr = unwrap!(parse_next(self), {
			return None;
		});

		while let Some(t) = self.get_token() {
			let op = unwrap!(map_op(&t.token_type), {
				self.put_token(t);
				break;
			});

			let right = unwrap!(parse_next(self), {
				break;
			});

			expr = Expression::Binary(Binary {
				left: Box::new(expr),
				op: op,
				right: Box::new(right)
			});
		} // while

		Some(expr)
	}

	fn parse_bitor(&mut self) -> Option<Expression> {
		trace!("parse_bitor");

		self.parse_binary(
			|tt| {
				match tt {
					&TokenType::Pipe => Some(BinaryOp::BitOr),
					_ => None
				}
			},
			|p| { p.parse_bitxor() }
		)
	} // parse_bitor

	fn parse_bitxor(&mut self) -> Option<Expression> {
		trace!("parse_bitxor");

		self.parse_binary(
			|tt| {
				match tt {
				&TokenType::Caret => Some(BinaryOp::BitXor),
					_ => None
				}
			},
			|p| { p.parse_bitand() }
		)
	} // parse_bitxor

	fn parse_bitand(&mut self) -> Option<Expression> {
		trace!("parse_bitand");

		self.parse_binary(
			|tt| {
				match tt {
				&TokenType::Ampersand => Some(BinaryOp::BitAnd),
					_ => None
				}
			},
			|p| { p.parse_shift() }
		)
	} // parse_bitand

	fn parse_shift(&mut self) -> Option<Expression> {
		trace!("parse_shift");

		self.parse_binary(
			|tt| {
				match tt {
				&TokenType::LeftAngleBracketX2 => Some(BinaryOp::LeftShift),
				&TokenType::RightAngleBracketX2 => Some(BinaryOp::RightShift),
					_ => None
				}
			},
			|p| { p.parse_add_subtract() }
		)
	} // parse_shift

	fn parse_add_subtract(&mut self) -> Option<Expression> {
		trace!("parse_add_subtract");

		self.parse_binary(
			|tt| {
				match tt {
				&TokenType::Plus => Some(BinaryOp::Plus),
				&TokenType::Minus => Some(BinaryOp::Minus),
					_ => None
				}
			},
			|p| { p.parse_multiply_divide() }
		)
	} // parse_add_subtract

	fn parse_multiply_divide(&mut self) -> Option<Expression> {
		trace!("parse_multiply_divide");

		self.parse_binary(
			|tt| {
				match tt {
				&TokenType::Star => Some(BinaryOp::Multiply),
				&TokenType::ForwardSlash => Some(BinaryOp::Divide),
				&TokenType::Percent => Some(BinaryOp::Modulo),
					_ => None
				}
			},
			|p| { p.parse_exponent() }
		)
	} // parse_multiply_divide

	fn parse_exponent(&mut self) -> Option<Expression> {
		trace!("parse_exponent");

		self.parse_binary(
			|tt| {
				match tt {
				&TokenType::StarX2 => Some(BinaryOp::Exponent),
					_ => None
				}
			},
			|p| { p.parse_unary() }
		)
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

		match self.parse_unary() {
			Some(right) => Some(Expression::Unary(Unary {
				op: op,
				right: Box::new(right)
			})),
			None => None,
		}
	} // parse_unary

	fn parse_primary(&mut self) -> Option<Expression> {
		trace!("parse_primary");

		let t = unwrap!(self.expect_token(), {
			return None;
		});

		match t.token_type {
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

pub fn parse<'a>(input: &'a str) -> Option<Ast> {
	Parser::new(input).parse_ast()
}

#[cfg(test)]
mod tests {
	use parsing::*;

	fn expect(input: &str, expected: Ast) {
		let result = unwrap!(parse(input), {
			panic!("Expected Ast but found None");
		});

		assert_eq!(result, expected);
	}

	#[test]
	fn parse_primary() {
		// literal
		expect("123", Ast::Expression(Expression::Literal(Literal::Number(123f64))));
		expect("123.456", Ast::Expression(Expression::Literal(Literal::Number(123.456f64))));
		expect("1_234.567", Ast::Expression(Expression::Literal(Literal::Number(1234.567f64))));

		// variable
		expect("e", Ast::Expression(Expression::Variable(Variable { name: "e".to_string() })));

		// parens
		expect("(123)", Ast::Expression(Expression::Literal(Literal::Number(123f64))));
		expect("(e)", Ast::Expression(Expression::Variable(Variable { name: "e".to_string() })));
	}

	#[test]
	fn parse_unary() {
		// negate
		expect("-123", Ast::Expression(Expression::Unary(Unary {
			op: UnaryOp::Negate,
			right: Box::new(Expression::Literal(Literal::Number(123f64))),
		})));

		// not
		expect("!e", Ast::Expression(Expression::Unary(Unary {
			op: UnaryOp::Not,
			right: Box::new(Expression::Variable(Variable { name: "e".to_string() })),
		})));
	}

	#[test]
	fn parse_binary() {
		// BitAnd
		expect("2&7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::BitAnd,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// BitOr
		expect("2|7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::BitOr,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// BitXor
		expect("2^7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::BitXor,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// Divide
		expect("2/7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::Divide,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// Exponent
		expect("2**7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::Exponent,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// LeftShift
		expect("2<<7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::LeftShift,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// Minus
		expect("2-7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::Minus,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// Modulo
		expect("2%7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::Modulo,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// Multiply
		expect("2*7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::Multiply,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// Plus
		expect("2+7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::Plus,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));

		// RightShift
		expect("2>>7", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(2f64))),
			op: BinaryOp::RightShift,
			right: Box::new(Expression::Literal(Literal::Number(7f64))),
		})));
	}

	#[test]
	fn parse_command() {
		expect("exit", Ast::Command(Command::Exit));
		expect("quit", Ast::Command(Command::Exit));
	}

	#[test]
	fn parse_pemdas() {
		expect("6/3-2", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Binary(Binary {
				left: Box::new(Expression::Literal(Literal::Number(6f64))),
				op: BinaryOp::Divide,
				right: Box::new(Expression::Literal(Literal::Number(3f64))),
			})),
			op: BinaryOp::Minus,
			right: Box::new(Expression::Literal(Literal::Number(2f64))),
		})));

		expect("6/(3-2)", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(6f64))),
			op: BinaryOp::Divide,
			right: Box::new(Expression::Binary(Binary {
				left: Box::new(Expression::Literal(Literal::Number(3f64))),
				op: BinaryOp::Minus,
				right: Box::new(Expression::Literal(Literal::Number(2f64))),
			})),
		})));

		expect("6*3**2", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Literal(Literal::Number(6f64))),
			op: BinaryOp::Multiply,
			right: Box::new(Expression::Binary(Binary {
				left: Box::new(Expression::Literal(Literal::Number(3f64))),
				op: BinaryOp::Exponent,
				right: Box::new(Expression::Literal(Literal::Number(2f64))),
			})),
		})));

		expect("(6*3)**2", Ast::Expression(Expression::Binary(Binary {
			left: Box::new(Expression::Binary(Binary {
				left: Box::new(Expression::Literal(Literal::Number(6f64))),
				op: BinaryOp::Multiply,
				right: Box::new(Expression::Literal(Literal::Number(3f64))),
			})),
			op: BinaryOp::Exponent,
			right: Box::new(Expression::Literal(Literal::Number(2f64))),
		})));
	}
} // mod tests
