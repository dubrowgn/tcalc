macro_rules! unwrap {
	($expr:expr, $fail_body:block) => (match $expr {
		::std::option::Option::Some(val) => val,
		::std::option::Option::None => $fail_body
	})
}

/* enable */

/*
macro_rules! trace {
	($fmt:expr) => (println!($fmt));
	($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
}
*/

/* disable */

macro_rules! trace {
	($fmt:expr) => ();
	($fmt:expr, $($arg:tt)*) => ();
}