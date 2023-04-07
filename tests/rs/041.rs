// Test 041 : CfgBoostError::MixedSyntaxError error.
use cfg_boost::{ target_cfg };


target_cfg! {

	#[cfg(unix)] & linux => {	// Test with simplified after legacy
		pub fn foo() -> String {
			String::from("Test 041 completed!")
		}
	}

}



fn main() {
    println!("{}", foo());
}
