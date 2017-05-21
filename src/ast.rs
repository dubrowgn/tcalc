#[derive(Debug)]
pub enum Ast {
	Command(Command),
	Expression(Expression)
}

#[derive(Debug)]
pub enum Command {
	Exit,
}

#[derive(Debug)]
pub enum Expression {
	Binary(Binary),
	Literal(Literal),
	Unary(Unary),
	Variable(Variable),
}

#[derive(Debug)]
pub struct Binary {
	pub left: Box<Expression>,
	pub op: BinaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum BinaryOp {
	BitAnd,
	BitOr,
	BitXor,
	Divide,
	Exponent,
	LeftShift,
	Minus,
	Modulo,
	Multiply,
	Plus,
	RightShift,
}

#[derive(Debug)]
pub struct Unary {
	pub op: UnaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum UnaryOp {
	Negate,
	Not,
}

#[derive(Debug)]
pub struct Variable {
	pub name: String
}

#[derive(Debug)]
pub enum Literal {
	Number(f64),
}
