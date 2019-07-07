extern crate dirs;
extern crate tcalc_rustyline;

use tcalc_rustyline::error::ReadlineError;
use tcalc_rustyline::Editor;
use std::env;

#[macro_use]
mod macros;

mod buffered_iterator;
mod ast;
mod scanning;
mod parsing;
mod running;

use ast::*;
use running::*;

fn print_usage() {
	println!("Usage: {} [OPTION] EXPRESSIONS", env!("CARGO_PKG_NAME"));
}

fn print_opts() {
	println!("Options:");
	println!("    --help              print this help menu");
	println!("    --version           print version information");
}

fn print_help() {
	print_usage();
	println!();
	print_opts();
}

fn print_try_help() {
	print_usage();
	println!();
	println!("Try '{} --help' for more information.", env!("CARGO_PKG_NAME"));
}

fn print_version() {
	println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn run_exprs<'a, I>(inputs: I) where I: Iterator<Item=String> {
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
	let mut rl = Editor::<()>::new();

	let history_path = match dirs::cache_dir() {
		Some(mut hist_dir) => {
			hist_dir.push("tcalc_history");
			Some(hist_dir)
		},
		_ => None
	};

	if let Some(ref path) = history_path {
		match rl.load_history(&path) { _ => { } }
	}

	loop {
		match rl.readline("> ") {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				match parsing::parse(&line) {
					Some(Ast::Command(Command::Exit)) => break,
					Some(Ast::Expression(expr)) => {
						match runner.run(expr) {
							Ok(v) => println!("  {}", v),
							Err(msg) => println!("{}", msg),
						}
					},
					None => {}
				} // match
			},
			Err(ReadlineError::Cancelled) => { },
			Err(ReadlineError::Interrupted) => break,
			Err(ReadlineError::Eof) => break,
			Err(msg) => println!("error: {}", msg),
		} // match
	} // loop

	if let Some(ref path) = history_path {
		match rl.save_history(&path) { _ => { } }
	}
} // repl

fn main() {
	let mut args = env::args();

	// start repl if there are no arguments
	if args.len() < 2 {
		repl();
		return;
	}

	// check for arguments
	let mut peekable = args.by_ref().skip(1).peekable();
	loop {
		match peekable.peek() {
			Some(arg) => match arg.as_str() {
				"--help" => {
					print_help();
					return;
				},
				"--version" => {
					print_version();
					return;
				},
				str => {
					if str.starts_with("--") {
						println!("Unrecognized option '{}'", str);
						println!("");
						print_try_help();
						return;
					}
					break;
				},
			}, // match
			_ => break
		} // match
	} // loop

	// evaluate remaining inputs
	run_exprs(peekable);
} // main
