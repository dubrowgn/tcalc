#[derive(Debug, PartialEq)]
pub enum Ast {
	Command(Command),
	Expression(Expression),
	Statement(Statement),
}

#[derive(Debug, PartialEq)]
pub enum Command {
	Exit,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
	Assignment(Assignment),
	Binary(Binary),
	Literal(Literal),
	Unary(Unary),
	Variable(Variable),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
	DeleteVar(Variable),
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
	pub var: Variable,
	pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Binary {
	pub left: Box<Expression>,
	pub op: BinaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Unary {
	pub op: UnaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
	Negate,
	Not,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
	pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
	Number(f64),
}
