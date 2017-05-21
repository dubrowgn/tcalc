#[derive(Debug)]
#[derive(PartialEq)]
pub enum Ast {
	Command(Command),
	Expression(Expression)
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command {
	Exit,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expression {
	Binary(Binary),
	Literal(Literal),
	Unary(Unary),
	Variable(Variable),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Binary {
	pub left: Box<Expression>,
	pub op: BinaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug)]
#[derive(PartialEq)]
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
#[derive(PartialEq)]
pub struct Unary {
	pub op: UnaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum UnaryOp {
	Negate,
	Not,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Variable {
	pub name: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Literal {
	Number(f64),
}
