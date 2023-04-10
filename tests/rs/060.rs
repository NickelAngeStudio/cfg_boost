// Test 060 : Modifier + and - on match_cfg!
use cfg_boost::{ match_cfg };


fn foo1() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from(""),
		+ #[cfg(any())] => String::from("Test"),
		_ => String::from(""),
	}
}

fn foo2() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from(""),
		+ #[cfg(any())] => String::from("060"),
		_ => String::from(""),
	}
}

fn foo3() -> String {
	match_cfg! {
		- #[cfg(all())] => String::from(""),
		+ #[cfg(any())] => String::from("completed"),
		_ => String::from(""),
	}
}

fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
