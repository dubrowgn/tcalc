extern crate getopts;
extern crate rustyline;

use getopts::Options;
use rustyline::error::ReadlineError;
use rustyline::Editor;
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
	let mut rl = Editor::<()>::new();

	let history_path = match env::home_dir() {
		Some(mut home_dir) => {
			home_dir.push(".tcalc_history");
			Some(home_dir)
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
			Err(ReadlineError::Interrupted) => { },
			Err(ReadlineError::Eof) => break,
			Err(msg) => println!("error: {}", msg),
		} // match
	} // loop

	if let Some(ref path) = history_path {
		match rl.save_history(&path) { _ => { } }
	}
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
