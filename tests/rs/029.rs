// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 029 : CfgBoostError::WildcardArmOnSingle error.
use cfg_boost::{ single_cfg };


single_cfg!{ _ => {
    pub fn completed() {
        println!("Test 029 completed!");
    }
}

}



fn main() {
    completed();
}