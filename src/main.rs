extern crate getopts;

use getopts::Options;
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
use running::*;

fn build_options() -> Options {
	let mut opts = Options::new();

	opts.optflag("", "help", "print this help menu");
	opts.optflag("", "version", "print version information");

	opts
}

fn print_usage() {
	print!("Usage: {} [OPTION] EXPRESSIONS", env!("CARGO_PKG_NAME"));
}

fn print_opts(opts: &Options) {
	print!("{}", opts.usage(""));
}

fn print_help(opts: &Options) {
	print_usage();
	print_opts(opts);
}

fn print_try_help() {
	print_usage();
	println!();
	println!("Try '{} --help' for more information.", env!("CARGO_PKG_NAME"));
}

fn print_version() {
	println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn run_exprs<'a, I>(inputs: I) where I: Iterator<Item=&'a String> {
	let mut runner = Runner::new();

	for str in inputs {
		match parsing::parse(&str) {
			Some(Ast::Expression(expr)) => {
				match runner.run(expr) {
					Ok(v) => println!("{}", v),
					Err(msg) => println!("{}", msg),
				}
			},
			_ => {}
		} // match
	} // for
} // run_exprs

fn repl() {
	let mut runner = Runner::new();

	loop {
		let mut line = String::new();

		print!("> ");
		io::stdout().flush().expect("Could not flush stdout");
		
		match io::stdin().read_line(&mut line) {
			Ok(_) => {
				match parsing::parse(&line) {
					Some(Ast::Command(Command::Exit)) => break,
					Some(Ast::Expression(expr)) => {
						match runner.run(expr) {
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
	let args: Vec<String> = env::args().collect();
	let opts = build_options();

	let matches = match opts.parse(&args[1..]) {
		Ok(ms) => ms,
		Err(msg) => {
			println!("{}", msg);
			print_try_help();
			return;
		},
	};

	if matches.opt_present("help") {
		print_help(&opts);
		return;
	}

	if matches.opt_present("version") {
		print_version();
		return;
	}

	if matches.free.is_empty() {
		repl();
	} else {
		run_exprs(matches.free.iter());
	}
} // main
