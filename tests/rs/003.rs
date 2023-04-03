// Test 003 : Empty nodes.
use cfg_boost::{ meta_cfg };


#[meta_cfg()]
fn foo() -> String {
    String::from("Test 003 completed!")
}


fn main() {
    println!("{}", foo());
}
