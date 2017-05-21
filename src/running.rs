use ast::*;
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

		Runner {
			scope: scope
		}
	}

	pub fn run(&mut self, expr: Expression) -> Result<f64, String> {
		let ans = self.run_expression(expr);

		if let Ok(val) = ans {
			self.scope.insert("ans".to_string(), val);
		}

		ans
	} // run

	fn run_expression(&self, expr: Expression) -> Result<f64, String> {
		match expr {
			Expression::Literal(l) => self.run_literal(l),
			Expression::Variable(v) => self.run_variable(v),
			Expression::Unary(u) => self.run_unary(u),
			Expression::Binary(b) => self.run_binary(b),
		}
	} // run_expression

	pub fn run_literal(&self, lit: Literal) -> Result<f64, String> {
		match lit {
			Literal::Number(n) => Ok(n),
		}
	} // run_literal

	pub fn run_variable(&self, var: Variable) -> Result<f64, String> {
		match self.scope.get(&var.name) {
			Some(val) => Ok(*val),
			None => Err(format!("Unexpected variable \"{}\"", var.name)),
		}
	} // run_variable

	pub fn run_unary(&self, un: Unary) -> Result<f64, String> {
		let r = self.run_expression(*un.right)?;

		match un.op {
			UnaryOp::Negate => Ok(-r),
			UnaryOp::Not => Ok(!(r as i64) as f64),
		}
	} // run_unary

	pub fn run_binary(&self, bin: Binary) -> Result<f64, String> {
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
				match r {
					0f64 => Err("Cannot divide by zero".to_string()),
					_ => Ok(l / r)
				}
			},
			BinaryOp::Modulo => {
				match r {
					0f64 => Err("Cannot divide by zero".to_string()),
					_ => Ok(l % r)
				}
			},
			BinaryOp::Exponent => Ok(l.powf(r)),
		}
	} // run_binary
} // Runner
