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
	Literal(Literal),
	Variable(Variable),
	Unary(Unary),
	Binary(Binary),
}

#[derive(Debug)]
pub struct Binary {
	pub left: Box<Expression>,
	pub op: BinaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum BinaryOp {
	Plus,
	Minus,
	Multiply,
	Divide,
	Modulo,
	Exponent,
}

#[derive(Debug)]
pub struct Unary {
	pub op: UnaryOp,
	pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum UnaryOp {
	Negate,
}

#[derive(Debug)]
pub struct Variable {
	pub name: String
}

#[derive(Debug)]
pub enum Literal {
	Number(f64),
}
