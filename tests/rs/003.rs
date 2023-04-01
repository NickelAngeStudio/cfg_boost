// Test 003 : Empty nodes.
use cfg_boost::{ attr_cfg };


#[attr_cfg()]
fn foo() -> String {
    String::from("Test 003 completed!")
}


fn main() {
    println!("{}", foo());
}
