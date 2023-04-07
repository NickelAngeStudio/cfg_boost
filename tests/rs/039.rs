// Test 039 : Legacy syntax meta_cfg!.
use cfg_boost::{ meta_cfg };

#[meta_cfg(#[cfg(unix)])]
pub fn foo() -> String {
	String::from("Test 039 completed!")
}


fn main() {
    println!("{}", foo());
}
