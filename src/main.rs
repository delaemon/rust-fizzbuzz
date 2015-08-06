fn main() {
	for n in 0..30 {
		// println!("{}", fizzbuzz1(n));
		println!("{}", fizzbuzz2(n));
	}
}

fn fizzbuzz1(n:i32) -> String {
	let fizz = if n % 3 == 0 { "fizz" } else { "" };
	let buzz = if n % 5 == 0 { "buzz" } else { "" };
	return n.to_string() + fizz + buzz;
}

fn fizzbuzz2(n:i32) -> String {
	let fizz = match n % 3 {
			0 => "fizz",
			_ => "",
		};
	let buzz = match n % 5 {
			0 => "buzz",
			_ => "",
		};
	return n.to_string() + fizz + buzz;
}
