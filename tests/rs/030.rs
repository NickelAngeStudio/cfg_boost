// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 030 : CfgBoostError::WildcardArmOnTarget corrected.
use cfg_boost::{ target_cfg };


target_cfg!{ 
	desktop => {
    	pub fn completed() {
    	    println!("Test 030 completed!");
   	 	}
	},
	!desktop => {
		pub fn completed2() {
    	    println!("Test 030 completed!");
   	 	}
	}
}




fn main() {
    completed();
}
