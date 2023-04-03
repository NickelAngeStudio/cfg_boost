// Test 006 : CfgBoostError::InvalidCharacter corrected.
use cfg_boost::{ meta_cfg };


#[meta_cfg(linux | windows | macos)]
fn foo() -> String {
    String::from("Test 006 completed!")
}


fn main() {
    println!("{}", foo());
}
