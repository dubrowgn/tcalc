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

fn main() {
	for arg in env::args() {
		println!("{}", arg);
	}

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
} // main
