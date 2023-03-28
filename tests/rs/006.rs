// Test 006 : CfgBoostError::InvalidCharacter corrected.
use cfg_boost::{ cfg_target };


#[cfg_target(linux | windows | macos)]
fn foo() -> String {
    String::from("Test 006 completed!")
}


fn main() {
    println!("{}", foo());
}
