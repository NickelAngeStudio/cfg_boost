// Test 017 : CfgBoostError::ContentSeparatorError.
use cfg_boost::{ target_cfg, match_cfg };

target_cfg!{
    
    x86_64:ar => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
     x86:ar = {
        pub fn foo3() -> String {
            String::from("Test")
        }
    }
    
}

fn foo2() -> String {
    match_cfg!{
        x86_64:ar > {
            String::from("017") 
        },
        _ => String::from("017")
    }
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), "completed!");
}
