// Test 005 : CfgBoostError::InvalidCharacter.
use cfg_boost::{ attr_cfg };


#[attr_cfg(linux ^ windows ^ macos)]
fn foo() -> String {
    String::from("Test 005 completed!")
}


fn main() {
    println!("{}", foo());
}
