// Test 007 : CfgBoostError::AliasNotFound.
use cfg_boost::{ attr_cfg, target_cfg, match_cfg };

target_cfg!{
    desktap => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
    _ => {}
}

fn foo2() -> String {
    match_cfg!{
        desktap => {
            String::from("007") 
        },
        _ => {}
    }
}

#[attr_cfg(desktap)]
fn foo3() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), foo3());
}
