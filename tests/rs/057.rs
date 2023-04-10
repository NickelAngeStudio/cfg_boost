// Test 057 : Modifier - on match_cfg!
use cfg_boost::{ match_cfg };


fn foo1() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from(""),
		_ => String::from("Test"),
	}
}

fn foo2() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from(""),
		_ => String::from("057"),
	}
}

fn foo3() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from(""),
		_ => String::from("completed"),
	}
}

fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
