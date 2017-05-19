macro_rules! unwrap {
	($expr:expr, $fail_body:block) => (match $expr {
		::std::option::Option::Some(val) => val,
		::std::option::Option::None => $fail_body
	})
}

#[cfg(feature="trace")]
macro_rules! trace {
	($fmt:expr) => (println!($fmt));
	($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
}

#[cfg(not(feature="trace"))]
macro_rules! trace {
	($fmt:expr) => ();
	($fmt:expr, $($arg:tt)*) => ();
}
