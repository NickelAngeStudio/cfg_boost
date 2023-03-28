// Test 005 : CfgBoostError::InvalidCharacter.
use cfg_boost::{ cfg_target };


#[cfg_target(linux ^ windows ^ macos)]
fn foo() -> String {
    String::from("Test 005 completed!")
}


fn main() {
    println!("{}", foo());
}
