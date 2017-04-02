use std::env;
use std::io;

mod scanning;

fn main() {
	for arg in env::args() {
		println!("{}", arg);
	}

	loop {
		let mut line = String::new();
		match io::stdin().read_line(&mut line) {
			Ok(_) => {
				match line.as_ref() {
					"quit\n" | "exit\n" => break,
					_ => {
						let scanner = scanning::Scanner::new(&line);
						for _ in scanner { }
					},
				}
			}
			Err(msg) => println!("error: {}", msg),
		}
	} // loop
} // main
