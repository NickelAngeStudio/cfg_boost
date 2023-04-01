// Test 010 : CfgBoostError::InvalidConfigurationPredicate corrected.
use cfg_boost::{ attr_cfg, target_cfg, match_cfg };

target_cfg!{
    x86_64:ar => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
    _ => {
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

#[attr_cfg(x86_64:ar)]
fn foo3() -> String {
    String::from("completed!")
}

#[attr_cfg(!x86_64:ar)]
fn foo3() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), foo3());
}
