// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 031 : CfgBoostError::SingleMultipleArms error.
use cfg_boost::{ single_cfg };


single_cfg!{ desktop => {
        pub fn completed() {
            println!("Test 031 completed!");
        }
    },
    mobile => {
        pub fn completed2() {
            println!("Test 031 completed!");
        }
    }
}



fn main() {
    completed();
}
