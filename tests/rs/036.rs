// Test 036 : Legacy syntax target_cfg!.
use cfg_boost::{ target_cfg };


target_cfg!{
    linux => {
        pub fn foo1() -> String {
            String::from("Test")
        }
    },
    #[cfg(unix)] => {
        pub fn foo2() -> String {
            String::from("036")
        }
    },
    #[cfg(target_os="linux")] => {
        pub fn foo3() -> String {
            String::from("completed")
        }
    },
}



fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
