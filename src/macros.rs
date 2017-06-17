macro_rules! unwrap {
	($expr:expr, $fail_body:block) => (
		match $expr {
			::std::option::Option::Some(val) => val,
			::std::option::Option::None => $fail_body
		}
	);
}

macro_rules! is_none {
	($expr:expr) => (
		match $expr {
			::std::option::Option::None => true,
			_ => false,
		}
	);
}

macro_rules! is_some {
	($expr:expr) => (
		match $expr {
			::std::option::Option::None => false,
			_ => true,
		}
	);
}

macro_rules! matches {
	($expr:expr, $($pattern:tt)+) => (
		match $expr {
			$($pattern)+ => true,
			_ => false,
		}
	);
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
