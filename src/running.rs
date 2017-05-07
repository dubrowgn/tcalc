use ast::*;

impl Expression {
	pub fn run(self) -> Result<f64, &'static str> {
		match self {
			Expression::Literal(l) => l.run(),
			Expression::Unary(u) => u.run(),
			Expression::Binary(b) => b.run(),
		}
	} // run
} // Expression

impl Binary {
	pub fn run(self) -> Result<f64, &'static str> {
		let l = self.left.run()?;
		let r = self.right.run()?;

		match self.op {
			BinaryOp::Plus => Ok(l + r),
			BinaryOp::Minus => Ok(l - r),
			BinaryOp::Multiply => Ok(l * r),
			BinaryOp::Divide => {
				match r {
					0f64 => Err("Cannot divide by zero"),
					_ => Ok(l / r)
				}
			},
			BinaryOp::Exponent => Ok(l.powf(r)),
		}
	} // run
} // Binary

impl Unary {
	pub fn run(self) -> Result<f64, &'static str> {
		let r = self.right.run()?;

		match self.op {
			UnaryOp::Negate => Ok(-r),
		}
	} // run
} // Unary

impl Literal {
	pub fn run(self) -> Result<f64, &'static str> {
		match self {
			Literal::Number(n) => Ok(n),
		}
	} // run
} // Literal
