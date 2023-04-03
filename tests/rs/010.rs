// Test 010 : CfgBoostError::InvalidConfigurationPredicate corrected.
use cfg_boost::{ meta_cfg, target_cfg, match_cfg };

target_cfg!{
    x86_64:ar => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    }
}

fn foo2() -> String {
    match_cfg!{
        x86_64:ar => {
            String::from("010") 
        },
        _ => String::from("010") 
    }
}

#[meta_cfg(x86_64:ar)]
fn foo3() -> String {
    String::from("completed!")
}

#[meta_cfg(!x86_64:ar)]
fn foo3() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), foo3());
}
