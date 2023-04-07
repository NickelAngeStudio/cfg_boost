// Test 003 : CfgBoostError::LegacySyntaxError error.
use cfg_boost::{ target_cfg };


target_cfg!{
    linux => {
        pub fn foo() -> String {
            String::from("Test 003 completed!")
        }
    },
    #(cfg(unix)) => {	// Incorrect legacy syntax
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
}



fn main() {
    println!("{}", foo());
}
