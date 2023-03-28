// Test 003 : Empty nodes.
use cfg_boost::{ cfg_target };


#[cfg_target()]
fn foo() -> String {
    String::from("Test 003 completed!")
}


fn main() {
    println!("{}", foo());
}
