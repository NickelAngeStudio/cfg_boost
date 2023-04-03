// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Performance target_cfg!.

use cfg_boost::target_cfg;

target_cfg!{
    desktop => {
        pub fn desktop_fn(){
            println!("perf_target_cfg completed!")
        }
    },
    mobile => {
        pub fn mobile_fn(){
            println!("perf_target_cfg completed!")
        }
    }
}

fn main() {
    desktop_fn();
}
