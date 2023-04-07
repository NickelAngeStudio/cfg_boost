// Test 003 : CfgBoostError::LegacySyntaxError corrected.
use cfg_boost::{ target_cfg };


target_cfg!{
    linux => {
        pub fn foo() -> String {
            String::from("Test 004 completed!")
        }
    },
    #[cfg(unix)] => {	// Corrected legacy syntax
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
}



fn main() {
    println!("{}", foo());
}
