use crate::ast::*;
use std::collections::HashMap;
use std::f64::consts::*;

pub struct Runner {
	scope: HashMap<String, f64>,
}

impl Runner {
	pub fn new() -> Runner {
		let mut scope = HashMap::new();

		scope.insert("e".to_string(), E);
		scope.insert("pi".to_string(), PI);

		Runner { scope }
	}

	pub fn run_expression(&mut self, expr: Expression) -> Result<f64, String> {
		let ans = match expr {
			Expression::Assignment(a) => self.run_assignment(a),
			Expression::Binary(b) => self.run_binary(b),
			Expression::Literal(l) => self.run_literal(l),
			Expression::Unary(u) => self.run_unary(u),
			Expression::Variable(v) => self.run_variable(v),
		}?;

		self.scope.insert("ans".to_string(), ans);

		Ok(ans)
	}

	pub fn run_statement(&mut self, stmt: Statement) -> Result<(), String> {
		match stmt {
			Statement::DeleteVar(var) => self.run_delete_var(var),
		}
	}

	fn run_literal(&self, lit: Literal) -> Result<f64, String> {
		match lit {
			Literal::Number(n) => Ok(n),
		}
	} // run_literal

	fn run_variable(&self, var: Variable) -> Result<f64, String> {
		match self.scope.get(&var.name) {
			Some(val) => Ok(*val),
			None => Err(format!("Variable \"{}\" is undefined", var.name)),
		}
	} // run_variable

	fn run_unary(&mut self, un: Unary) -> Result<f64, String> {
		let r = self.run_expression(*un.right)?;

		match un.op {
			UnaryOp::Negate => Ok(-r),
			UnaryOp::Not => Ok(!(r as i64) as f64),
		}
	} // run_unary

	fn run_binary(&mut self, bin: Binary) -> Result<f64, String> {
		let l = self.run_expression(*bin.left)?;
		let r = self.run_expression(*bin.right)?;

		match bin.op {
			BinaryOp::BitAnd => Ok(((l as i64) & (r as i64)) as f64),
			BinaryOp::BitOr => Ok(((l as i64) | (r as i64)) as f64),
			BinaryOp::BitXor => Ok(((l as i64) ^ (r as i64)) as f64),
			BinaryOp::LeftShift => Ok(((l as i64) << (r as i64)) as f64),
			BinaryOp::RightShift => Ok(((l as i64) >> (r as i64)) as f64),
			BinaryOp::Plus => Ok(l + r),
			BinaryOp::Minus => Ok(l - r),
			BinaryOp::Multiply => Ok(l * r),
			BinaryOp::Divide => {
				if r == 0f64 {
					Err("Cannot divide by zero".to_string())
				} else {
					Ok(l / r)
				}
			}
			BinaryOp::Modulo => {
				if r == 0f64 {
					Err("Cannot divide by zero".to_string())
				} else {
					Ok(l % r)
				}
			}
			BinaryOp::Exponent => Ok(l.powf(r)),
		}
	} // run_binary

	fn run_assignment(&mut self, assign: Assignment) -> Result<f64, String> {
		let r = self.run_expression(*assign.right)?;
		self.scope.insert(assign.var.name, r);
		Ok(r)
	} // run_assignment

	fn run_delete_var(&mut self, var: Variable) -> Result<(), String> {
		if self.scope.remove(&var.name) == None {
			Err(format!("Variable \"{}\" is undefined", var.name))
		} else {
			Ok(())
		}
	}
} // Runner

#[cfg(test)]
mod tests {
	use crate::parsing::*;
	use crate::running::*;

	fn solve(input: &str) -> f64 {
		let ast = unwrap!(parse(input), {
			panic!("Expected Ast for input \"{}\", but found None", input);
		});

		let expr = match ast {
			Ast::Expression(expr) => expr,
			_ => panic!(
				"Expected Expression for input \"{}\", but found {:?}",
				input, ast
			),
		};

		match Runner::new().run_expression(expr) {
			Ok(v) => v,
			Err(msg) => panic!("Error for input \"{}\": {}", input, msg),
		}
	} // solve

	#[test]
	fn solve_literal() {
		assert_eq!(solve("123"), 123f64);
		assert_eq!(solve("123.456"), 123.456f64);
		assert_eq!(solve("1_234.567"), 1_234.567f64);
	}

	#[test]
	fn solve_assignment() {
		assert_eq!(solve("a=123"), 123f64);
	}

	#[test]
	fn solve_variable() {
		assert_eq!(solve("e"), E);
	}

	#[test]
	fn solve_parens() {
		assert_eq!(solve("(123)"), 123f64);
		assert_eq!(solve("(pi)"), PI);
	}

	#[test]
	fn solve_unary_ops() {
		assert_eq!(solve("-123"), -123f64);
		assert_eq!(solve("!e"), !(E as i64) as f64);
	}

	#[test]
	fn solve_binary_ops() {
		assert_eq!(solve("2&7"), 2f64);
		assert_eq!(solve("2|7"), 7f64);
		assert_eq!(solve("2^7"), 5f64);
		assert_eq!(solve("2/7"), 2f64 / 7f64);
		assert_eq!(solve("2**7"), 128f64);
		assert_eq!(solve("2<<7"), 256f64);
		assert_eq!(solve("2-7"), -5f64);
		assert_eq!(solve("2%7"), 2f64);
		assert_eq!(solve("2*7"), 14f64);
		assert_eq!(solve("2+7"), 9f64);
		assert_eq!(solve("2>>7"), 0f64);
	}

	#[test]
	fn solve_pemdas() {
		assert_eq!(solve("6/3-2"), 0f64);
		assert_eq!(solve("6/(3-2)"), 6f64);
		assert_eq!(solve("6*3**2"), 54f64);
		assert_eq!(solve("(6*3)**2"), 324f64);
	}
} // mod tests
