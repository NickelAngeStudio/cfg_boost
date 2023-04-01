// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 032 : CfgBoostError::SingleMultipleArms corrected.
use cfg_boost::{ single_cfg };


single_cfg!{ desktop => {
        pub fn completed() {
            println!("Test 032 completed!");
        }
    }
}

single_cfg!{ mobile => compile_error!("Not for mobile!"); }


fn main() {
    completed();
}
