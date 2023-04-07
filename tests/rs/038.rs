// Test 038 : Legacy syntax match_cfg! wildcard.
use cfg_boost::{ match_cfg };


pub fn foo() -> String {
	match_cfg!{
		#[cfg(windows)] => String::from("error"),
		#[cfg(wasm)] => String::from("error"),
		_ => String::from("Test 038 completed!"),
	}
}


fn main() {
    println!("{}", foo());
}
