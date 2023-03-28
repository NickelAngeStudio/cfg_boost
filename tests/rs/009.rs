// Test 009 : CfgBoostError::InvalidConfigurationPredicate.
use cfg_boost::{ cfg_target, target_cfg, match_cfg };

target_cfg!{
    x64_64:aa => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
    _ => {}
}

fn foo2() -> String {
    match_cfg!{
        x64_64:aa => {
            String::from("008") 
        },
        _ => {}
    }
}

#[cfg_target(x64_64:aa)]
fn foo3() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), foo3());
}
