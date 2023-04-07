// Test 040 : CfgBoostError::MixedSyntaxError error.
use cfg_boost::{ target_cfg };


target_cfg! {

	linux & #[cfg(unix)] => {	// Test with simplified before legacy
		pub fn foo() -> String {
			String::from("Test 040 completed!")
		}
	}

}



fn main() {
    println!("{}", foo());
}
