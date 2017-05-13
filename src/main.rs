use std::env;
use std::io;
use std::io::Write;

#[macro_use]
mod macros;

mod buffered_iterator;
mod ast;
mod scanning;
mod parsing;
mod running;

use ast::*;

fn parse_args() {
	for arg in env::args().skip(1) {
		match parsing::parse(&arg) {
			Some(Ast::Expression(expr)) => {
				match expr.run() {
					Ok(v) => println!("{}", v),
					Err(msg) => println!("{}", msg),
				}
			},
			_ => {}
		} // match
	} // for (arg)
} // parse_args

fn repl() {
	loop {
		let mut line = String::new();

		print!("> ");
		io::stdout().flush().expect("Could not flush stdout");
		
		match io::stdin().read_line(&mut line) {
			Ok(_) => {
				match parsing::parse(&line) {
					Some(Ast::Command(Command::Exit)) => break,
					Some(Ast::Expression(expr)) => {
						match expr.run() {
							Ok(v) => println!("  {}", v),
							Err(msg) => println!("{}", msg),
						}
					},
					None => {}
				}
			}
			Err(msg) => println!("error: {}", msg),
		}
	} // loop
} // repl

fn main() {
	if env::args().len() > 1 {
		return parse_args();
	}

	repl();
} // main
