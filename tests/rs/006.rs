// Test 006 : CfgBoostError::InvalidCharacter corrected.
use cfg_boost::{ attr_cfg };


#[attr_cfg(linux | windows | macos)]
fn foo() -> String {
    String::from("Test 006 completed!")
}


fn main() {
    println!("{}", foo());
}
