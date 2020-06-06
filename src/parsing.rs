use crate::ast::*;
use crate::buffered_iterator::*;
use crate::scanning::*;

struct Parser<'a> {
	scanner: BufferedIterator<Token, Scanner<'a>>,
}

impl<'a> Parser<'a> {
	fn new(input: &str) -> Parser<'_> {
		let scanner = Scanner::new(input);
		let buf = BufferedIterator::new(scanner);

		Parser { scanner: buf }
	}

	fn unexpected_token(&self, found: &Token) {
		println!(
			"Unexpected token '{:?}' (line {}, col {})",
			found.token_type, found.line, found.column
		);
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

	fn expect_terminal(&mut self) -> bool {
		// end of input
		let t = unwrap!(self.get_token(), {
			return true;
		});

		// explicit terminal characters
		match t.token_type {
			TokenType::NewLine => true,
			_ => {
				self.unexpected_token(&t);
				false
			}
		}
	} // expect_terminal

	fn parse_ast(&mut self) -> Option<Ast> {
		trace!("parse_ast");

		let ast = if let Some(cmd) = self.parse_command() {
			Some(Ast::Command(cmd))
		} else if let Some(stmt) = self.parse_statement() {
			Some(Ast::Statement(stmt))
		} else if let Some(expr) = self.parse_expression() {
			Some(Ast::Expression(expr))
		} else {
			None
		};

		if !self.expect_terminal() {
			return None;
		}

		ast
	}

	fn parse_command(&mut self) -> Option<Command> {
		trace!("parse_command");

		let t = unwrap!(self.get_token(), {
			return None;
		});

		if let Token {
			token_type: TokenType::Identifier { ref str, .. },
			..
		} = t
		{
			match str.as_str() {
				"exit" | "quit" => return Some(Command::Exit),
				_ => {}
			}
		}

		self.put_token(t);

		None
	} // parse_command

	fn parse_statement(&mut self) -> Option<Statement> {
		trace!("parse_statement");

		let t = unwrap!(self.get_token(), {
			return None;
		});

		if let Token {
			token_type: TokenType::Identifier { ref str, .. },
			..
		} = t
		{
			if let "delete" = str.as_str() {
				let tvar = unwrap!(self.expect_token(), {
					return None;
				});
				if let TokenType::Identifier { str } = tvar.token_type {
					return Some(Statement::DeleteVar(Variable { name: str }));
				}
				self.put_token(tvar);
			}
		}

		self.put_token(t);

		None
	}

	fn parse_expression(&mut self) -> Option<Expression> {
		trace!("parse_expression");

		self.parse_assign()
	} // parse_expression

	fn parse_assign(&mut self) -> Option<Expression> {
		trace!("parse_assign");

		// parse expression
		let expr = self.parse_bitor();

		// check if it was a variable
		let var = if let Some(Expression::Variable(var)) = expr {
			var
		} else {
			return expr;
		};

		// check if the next token is an equal sign
		let op_opt: Option<BinaryOp>;
		let mut right_opt: Option<Expression> = None;
		if let Some(t) = self.get_token() {
			match t.token_type {
				TokenType::AmpersandEqual => op_opt = Some(BinaryOp::BitAnd),
				TokenType::CaretEqual => op_opt = Some(BinaryOp::BitXor),
				TokenType::Equal => op_opt = None,
				TokenType::ForwardSlashEqual => op_opt = Some(BinaryOp::Divide),
				TokenType::LeftAngleBracketX2Equal => op_opt = Some(BinaryOp::LeftShift),
				TokenType::MinusEqual => op_opt = Some(BinaryOp::Minus),
				TokenType::MinusX2 => {
					op_opt = Some(BinaryOp::Minus);
					right_opt = Some(Expression::Literal(Literal::Number(1f64)));
				}
				TokenType::PercentEqual => op_opt = Some(BinaryOp::Modulo),
				TokenType::PipeEqual => op_opt = Some(BinaryOp::BitOr),
				TokenType::PlusEqual => op_opt = Some(BinaryOp::Plus),
				TokenType::PlusX2 => {
					op_opt = Some(BinaryOp::Plus);
					right_opt = Some(Expression::Literal(Literal::Number(1f64)));
				}
				TokenType::RightAngleBracketX2Equal => op_opt = Some(BinaryOp::RightShift),
				TokenType::StarEqual => op_opt = Some(BinaryOp::Multiply),
				TokenType::StarX2Equal => op_opt = Some(BinaryOp::Exponent),
				_ => {
					self.put_token(t);
					return Some(Expression::Variable(var));
				}
			}
		} else {
			return Some(Expression::Variable(var));
		}

		if right_opt.is_none() {
			right_opt = self.parse_assign();
		}

		// parse the right-hand expression
		let mut right = unwrap!(right_opt, {
			println!("Missing right-hand side of assignment to \"{}\"", var.name);
			return None;
		});

		if let Some(op) = op_opt {
			right = Expression::Binary(Binary {
				left: Box::new(Expression::Variable(Variable {
					name: var.name.clone(),
				})),
				op,
				right: Box::new(right),
			});
		}

		Some(Expression::Assignment(Assignment {
			var,
			right: Box::new(right),
		}))
	} // parse_assign

	fn parse_binary<FOp, FNext>(&mut self, map_op: FOp, parse_next: FNext) -> Option<Expression>
	where
		FOp: Fn(&TokenType) -> Option<BinaryOp>,
		FNext: Fn(&mut Parser<'_>) -> Option<Expression>,
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
				op,
				right: Box::new(right),
			});
		} // while

		Some(expr)
	}

	fn parse_bitor(&mut self) -> Option<Expression> {
		trace!("parse_bitor");

		self.parse_binary(
			|tt| match tt {
				TokenType::Pipe => Some(BinaryOp::BitOr),
				_ => None,
			},
			|p| p.parse_bitxor(),
		)
	} // parse_bitor

	fn parse_bitxor(&mut self) -> Option<Expression> {
		trace!("parse_bitxor");

		self.parse_binary(
			|tt| match tt {
				TokenType::Caret => Some(BinaryOp::BitXor),
				_ => None,
			},
			|p| p.parse_bitand(),
		)
	} // parse_bitxor

	fn parse_bitand(&mut self) -> Option<Expression> {
		trace!("parse_bitand");

		self.parse_binary(
			|tt| match tt {
				TokenType::Ampersand => Some(BinaryOp::BitAnd),
				_ => None,
			},
			|p| p.parse_shift(),
		)
	} // parse_bitand

	fn parse_shift(&mut self) -> Option<Expression> {
		trace!("parse_shift");

		self.parse_binary(
			|tt| match tt {
				TokenType::LeftAngleBracketX2 => Some(BinaryOp::LeftShift),
				TokenType::RightAngleBracketX2 => Some(BinaryOp::RightShift),
				_ => None,
			},
			|p| p.parse_add_subtract(),
		)
	} // parse_shift

	fn parse_add_subtract(&mut self) -> Option<Expression> {
		trace!("parse_add_subtract");

		self.parse_binary(
			|tt| match tt {
				TokenType::Plus => Some(BinaryOp::Plus),
				TokenType::Minus => Some(BinaryOp::Minus),
				_ => None,
			},
			|p| p.parse_multiply_divide(),
		)
	} // parse_add_subtract

	fn parse_multiply_divide(&mut self) -> Option<Expression> {
		trace!("parse_multiply_divide");

		self.parse_binary(
			|tt| match tt {
				TokenType::Star => Some(BinaryOp::Multiply),
				TokenType::ForwardSlash => Some(BinaryOp::Divide),
				TokenType::Percent => Some(BinaryOp::Modulo),
				_ => None,
			},
			|p| p.parse_exponent(),
		)
	} // parse_multiply_divide

	fn parse_exponent(&mut self) -> Option<Expression> {
		trace!("parse_exponent");

		self.parse_binary(
			|tt| match tt {
				TokenType::StarX2 => Some(BinaryOp::Exponent),
				_ => None,
			},
			|p| p.parse_unary(),
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
				op,
				right: Box::new(right),
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
			TokenType::Number { str, prefix } => {
				let radix = match prefix.as_str() {
					"0b" => 2,
					"0o" => 8,
					"0d" => 10,
					"0x" => 16,
					_ => 10,
				};

				// rust core does not currently support parsing non-base-10 decimal numbers
				if radix == 10 {
					match str.parse::<f64>() {
						Ok(n) => Some(Expression::Literal(Literal::Number(n))),
						Err(msg) => {
							println!("Failed to parse number \"{}{}\": {}", prefix, str, msg);
							None
						}
					}
				} else {
					match u64::from_str_radix(str.as_str(), radix) {
						Ok(n) => Some(Expression::Literal(Literal::Number(n as f64))),
						Err(msg) => {
							println!("Failed to parse number \"{}{}\": {}", prefix, str, msg);
							None
						}
					}
				}
			}
			TokenType::Identifier { str } => Some(Expression::Variable(Variable { name: str })),
			TokenType::LeftParen => {
				let expr = self.parse_expression();
				match self.expect_token() {
					Some(Token {
						token_type: TokenType::RightParen,
						..
					}) => expr,
					Some(t) => {
						self.unexpected_token(&t);
						None
					}
					None => None,
				}
			}
			_ => {
				println!(
					"Unexpected token '{:?}' (line {}, col {})",
					t.token_type, t.line, t.column
				);
				None
			}
		} // match
	} // parse_primary
} // Parser

pub fn parse(input: &str) -> Option<Ast> {
	Parser::new(input).parse_ast()
}

#[cfg(test)]
mod tests {
	use crate::parsing::*;

	fn expect(input: &str, expected: Ast) {
		let result = unwrap!(parse(input), {
			panic!("Expected Ast for input \"{}\", but found None", input);
		});

		assert_eq!(result, expected);
	}

	fn assign_expr(name: &str, right: Expression) -> Expression {
		Expression::Assignment(Assignment {
			var: var(name),
			right: Box::new(right),
		})
	}

	fn assign_ast(name: &str, right: Expression) -> Ast {
		Ast::Expression(assign_expr(name, right))
	}

	fn bin_op_expr(left: Expression, op: BinaryOp, right: Expression) -> Expression {
		Expression::Binary(Binary {
			left: Box::new(left),
			op,
			right: Box::new(right),
		})
	}

	fn bin_op_ast(left: Expression, op: BinaryOp, right: Expression) -> Ast {
		Ast::Expression(bin_op_expr(left, op, right))
	}

	fn comp_assign_ast(name: &str, op: BinaryOp, right: Expression) -> Ast {
		assign_ast(name, bin_op_expr(var_expr(name), op, right))
	}

	fn num_expr(value: f64) -> Expression {
		Expression::Literal(Literal::Number(value))
	}

	fn num_ast(value: f64) -> Ast {
		Ast::Expression(num_expr(value))
	}

	fn uni_op_expr(op: UnaryOp, right: Expression) -> Expression {
		Expression::Unary(Unary {
			op,
			right: Box::new(right),
		})
	}

	fn uni_op_ast(op: UnaryOp, right: Expression) -> Ast {
		Ast::Expression(uni_op_expr(op, right))
	}

	fn var(name: &str) -> Variable {
		Variable {
			name: name.to_string(),
		}
	}

	fn var_expr(name: &str) -> Expression {
		Expression::Variable(var(name))
	}

	fn var_ast(name: &str) -> Ast {
		Ast::Expression(var_expr(name))
	}

	#[test]
	fn parse_literal() {
		expect("0b11", num_ast(3f64));
		expect("0o11", num_ast(9f64));
		expect("0x11", num_ast(17f64));
		expect("11", num_ast(11f64));
		expect("0_123_456_789", num_ast(123_456_789f64));
		expect("12345.67890", num_ast(12345.6789f64));
	}

	#[test]
	fn parse_variable() {
		expect("e", var_ast("e"));
	}

	#[test]
	fn parse_parens() {
		expect("(123)", num_ast(123f64));
		expect("(e)", var_ast("e"));
	}

	#[test]
	fn parse_negate() {
		expect("-123", uni_op_ast(UnaryOp::Negate, num_expr(123f64)));
	}

	#[test]
	fn parse_not() {
		expect("!e", uni_op_ast(UnaryOp::Not, var_expr("e")));
	}

	fn parse_bin_op(str_op: &str, op: BinaryOp) {
		expect(
			&format!("2 {} 7", str_op),
			bin_op_ast(num_expr(2f64), op, num_expr(7f64)),
		);
	}

	fn parse_comp_assign(str_op: &str, op: BinaryOp) {
		expect(
			&format!("a {} 7", str_op),
			comp_assign_ast("a", op, num_expr(7f64)),
		);
	}

	#[test]
	fn parse_bit_and() {
		parse_bin_op("&", BinaryOp::BitAnd);
	}

	#[test]
	fn parse_bit_and_assign() {
		parse_comp_assign("&=", BinaryOp::BitAnd);
	}

	#[test]
	fn parse_bit_or() {
		parse_bin_op("|", BinaryOp::BitOr);
	}

	#[test]
	fn parse_bit_or_assign() {
		parse_comp_assign("|=", BinaryOp::BitOr);
	}

	#[test]
	fn parse_bit_xor() {
		parse_bin_op("^", BinaryOp::BitXor);
	}

	#[test]
	fn parse_bit_xor_assign() {
		parse_comp_assign("^=", BinaryOp::BitXor);
	}

	#[test]
	fn parse_decrement() {
		expect("a--", comp_assign_ast("a", BinaryOp::Minus, num_expr(1f64)))
	}

	#[test]
	fn parse_divide() {
		parse_bin_op("/", BinaryOp::Divide);
	}

	#[test]
	fn parse_divide_assign() {
		parse_comp_assign("/=", BinaryOp::Divide);
	}

	#[test]
	fn parse_exponent() {
		parse_bin_op("**", BinaryOp::Exponent);
	}

	#[test]
	fn parse_exponent_assign() {
		parse_comp_assign("**=", BinaryOp::Exponent);
	}

	#[test]
	fn parse_increment() {
		expect("a++", comp_assign_ast("a", BinaryOp::Plus, num_expr(1f64)))
	}

	#[test]
	fn parse_left_shift() {
		parse_bin_op("<<", BinaryOp::LeftShift);
	}

	#[test]
	fn parse_left_shift_assign() {
		parse_comp_assign("<<=", BinaryOp::LeftShift);
	}

	#[test]
	fn parse_minus() {
		parse_bin_op("-", BinaryOp::Minus);
	}

	#[test]
	fn parse_minus_assign() {
		parse_comp_assign("-=", BinaryOp::Minus);
	}

	#[test]
	fn parse_modulo() {
		parse_bin_op("%", BinaryOp::Modulo);
	}

	#[test]
	fn parse_modulo_assign() {
		parse_comp_assign("%=", BinaryOp::Modulo);
	}

	#[test]
	fn parse_multiply() {
		parse_bin_op("*", BinaryOp::Multiply);
	}

	#[test]
	fn parse_multiply_assign() {
		parse_comp_assign("*=", BinaryOp::Multiply);
	}

	#[test]
	fn parse_plus() {
		parse_bin_op("+", BinaryOp::Plus);
	}

	#[test]
	fn parse_plus_assign() {
		parse_comp_assign("+=", BinaryOp::Plus);
	}

	#[test]
	fn parse_right_shift() {
		parse_bin_op(">>", BinaryOp::RightShift);
	}

	#[test]
	fn parse_right_shift_assign() {
		parse_comp_assign(">>=", BinaryOp::RightShift);
	}

	#[test]
	fn parse_assignment() {
		expect("a=8", assign_ast("a", num_expr(8f64)));
	}

	#[test]
	fn parse_command() {
		expect("exit", Ast::Command(Command::Exit));
		expect("quit", Ast::Command(Command::Exit));
	}

	#[test]
	fn parse_pemdas() {
		expect(
			"6/3-2",
			bin_op_ast(
				bin_op_expr(num_expr(6f64), BinaryOp::Divide, num_expr(3f64)),
				BinaryOp::Minus,
				num_expr(2f64),
			),
		);

		expect(
			"6/(3-2)",
			bin_op_ast(
				num_expr(6f64),
				BinaryOp::Divide,
				bin_op_expr(num_expr(3f64), BinaryOp::Minus, num_expr(2f64)),
			),
		);

		expect(
			"6*3**2",
			bin_op_ast(
				num_expr(6f64),
				BinaryOp::Multiply,
				bin_op_expr(num_expr(3f64), BinaryOp::Exponent, num_expr(2f64)),
			),
		);

		expect(
			"(6*3)**2",
			bin_op_ast(
				bin_op_expr(num_expr(6f64), BinaryOp::Multiply, num_expr(3f64)),
				BinaryOp::Exponent,
				num_expr(2f64),
			),
		);
	}

	#[test]
	fn parse_unexpected_terminal() {
		assert_eq!(parse("1+2)"), None);
	}
} // mod tests
