// Test 063 : Modifier @ removed
use cfg_boost::{ match_cfg };


fn foo1() -> String {
	match_cfg! {
		+ #[cfg(any())] => String::from("Test"),
		_ => String::from(""),
	}
}

fn foo2() -> String {
	match_cfg! {
		+ #[cfg(any())] => String::from("063"),
		_ => String::from(""),
	}
}

fn foo3() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from("failed"),
		_ => String::from("completed"),
	}
}

fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
