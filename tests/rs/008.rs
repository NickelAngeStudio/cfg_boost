// Test 008 : CfgBoostError::AliasNotFound corrected.
use cfg_boost::{ meta_cfg, target_cfg, match_cfg };

target_cfg!{
    desktop => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
}

fn foo2() -> String {
    match_cfg!{
        desktop => {
            String::from("008") 
        },
        _ => {}
    }
}

#[meta_cfg(desktop)]
fn foo3() -> String {
    String::from("completed!")
}


fn main() {
    println!("{} {} {}", foo1(), foo2(), foo3());
}
