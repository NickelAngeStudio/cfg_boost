// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 029 : CfgBoostError::WildcardArmOnTarget error.
use cfg_boost::{ target_cfg };


target_cfg!{ 
	desktop => {
    	pub fn completed() {
    	    println!("Test 029 completed!");
   	 	}
	},
	!desktop => {
		pub fn completed2() {
    	    println!("Test 029 completed!");
   	 	}
	},
	_ => {
		pub fn completed3() {
    	    println!("Test 029 completed!");
   	 	}
	}

}



fn main() {
    completed();
}
