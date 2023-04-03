// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 033 : CfgBoostError::TargetInFunction corrected.
use cfg_boost::{ match_cfg };


pub fn completed() {
	let a = match_cfg!{ 
		desktop => 5,
		!desktop => 10,
		_ => 20
	};
	
	println!("a={}", a);
	println!("Test 033 completed!");
}

fn main() {
    completed();
}
