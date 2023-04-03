// Test 009 : CfgBoostError::InvalidConfigurationPredicate.
use cfg_boost::{ meta_cfg, target_cfg, match_cfg };

target_cfg!{
    x86_64:aa => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    }
}

fn foo2() -> String {
    match_cfg!{
        x86_64:aa => {
            String::from("009") 
        },
        _ => String::from("009") 
    }
}

#[meta_cfg(x86_64:aa)]
fn foo3() -> String {
    String::from("completed!")
}

#[meta_cfg(!x86_64:aa)]
fn foo3() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), foo3());
}
