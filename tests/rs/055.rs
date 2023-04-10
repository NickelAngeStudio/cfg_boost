// Test 055 : Modifier + on meta_cfg
use cfg_boost::{ meta_cfg };


#[meta_cfg(+ #[cfg(any())])]
pub fn foo1() -> String {
	String::from("Test")
}

#[meta_cfg(+ foo:os)]
pub fn foo2() -> String {
	String::from("055")
}

#[meta_cfg(+ foo:ft)]
pub fn foo3() -> String {
	String::from("completed")
}

fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
