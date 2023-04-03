// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 032 : CfgBoostError::TargetInFunction error.
use cfg_boost::{ target_cfg };


pub fn completed() {
	target_cfg!{ 
		desktop => {
			let a = 5;
		},
		!desktop => {
			let a = 10;
		}
	}
	
	println!("a={}", a);
	println!("Test 032 completed");
}

fn main() {
    completed();
}
