// Test 004 : Empty nodes error corrected.
use cfg_boost::{ cfg_target };


#[cfg_target(desktop)]
fn foo() -> String {
    String::from("Test 004 completed!")
}


fn main() {
    println!("{}", foo());
}
