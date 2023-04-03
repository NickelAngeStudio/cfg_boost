#![cfg_attr(docsrs, feature(doc_cfg))]

use cfg_boost::target_cfg;

target_cfg!{
    desktop => {
        pub fn desktop_fn(){
            println!("Completed!")
        }
        
        pub fn desktop2_fn(){
            println!("Completed!")
        }
    },
    mobile => {
        pub fn mobile_fn(){
            println!("Completed!")
        }
    }
}

fn main() {
    desktop_fn();
}
