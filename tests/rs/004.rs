// Test 004 : Empty nodes error corrected.
use cfg_boost::{ attr_cfg };


#[attr_cfg(desktop)]
fn foo() -> String {
    String::from("Test 004 completed!")
}


fn main() {
    println!("{}", foo());
}
