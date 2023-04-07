// Test 037 : Legacy syntax match_cfg! not wildcard.
use cfg_boost::{ match_cfg };


pub fn foo() -> String{
	match_cfg!{
		#[cfg(windows)] => String::from("error"),
		#[cfg(unix)] => String::from("Test 037 completed!"),
		_ => String::from("error")
	}
}


fn main() {
    println!("{}", foo());
}
