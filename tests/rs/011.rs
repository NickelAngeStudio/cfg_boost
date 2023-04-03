// Test 011 : CfgBoostError::EmptyArm.
use cfg_boost::{ target_cfg, match_cfg };

target_cfg!{
     => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
}

fn foo2() -> String {
    match_cfg!{
         => {
            String::from("011") 
        },
        _ => String::from("011") 
    }
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), "completed");
}
