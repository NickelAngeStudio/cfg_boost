// Test 021 : Predefined predicates.
use cfg_boost::{ target_cfg, match_cfg, cfg_target };

/**************
 * TARGET_CFG *
 **************/
// ar : target architecture
target_cfg!{
    
    x86_64:ar => {
        pub fn tar_foo() -> String {
            String::from("tar:x86_64")
        }
    },
    x86:ar => {
        pub fn tar_foo() -> String {
            String::from("tar:x86")
        }
    },
    _ => {
        pub fn tar_foo() -> String {
            String::from("tar:unknown")
        }
    },
    
}

// tf : target architecture feature

// os : target operating system

// fm : target family

// ev : target environment

// ed : target endian

// pw : target pointer width

// vn : target vendor

// at : target has atomic

// pn : panic feature

// ft : feature


/*************
 * MATCH_CFG *
 *************/
// ar : target architecture
fn mar_foo() -> String {
    match_cfg!{
        x86_64:ar => String::from("mar:x86_64"),
        x86:ar => String::from("mar:x86"),
        _ => String::from("mar:unknown")
    }
}

// tf : target architecture feature

// os : target operating system

// fm : target family

// ev : target environment

// ed : target endian

// pw : target pointer width

// vn : target vendor

// at : target has atomic

// pn : panic feature

// ft : feature

/**************
 * CFG_TARGET *
 **************/
// ar : target architecture
#[cfg_target(x86_64:ar)]
fn car_foo() -> String {
    String::from("car:x86_64")
}
#[cfg_target(x86:ar)]
fn car_foo() -> String {
    String::from("car:x86_64")
}
#[cfg_target(!(x86_64:ar | x86:ar))]
fn car_foo() -> String {
    String::from("car:unknown")
}

// tf : target architecture feature

// os : target operating system

// fm : target family

// ev : target environment

// ed : target endian

// pw : target pointer width

// vn : target vendor

// at : target has atomic

// pn : panic feature

// ft : feature


fn main() {
    println!("{} {} {}", tar_foo(), mar_foo(), car_foo());
}
