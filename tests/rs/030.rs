// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 030 : CfgBoostError::WildcardArmOnSingle corrected.
use cfg_boost::{ single_cfg };


single_cfg!{ desktop => {
    pub fn completed() {
        println!("Test 030 completed!");
    }
}

}



fn main() {
    completed();
}
