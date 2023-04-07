// Test 043 : CfgBoostError::ContentSeparatorMissing error.
use cfg_boost::{ target_cfg };


target_cfg! {

	linux {},
	#[cfg(unix)] => {
		pub fn foo() -> String {
			String::from("Test 043 completed!")
		}
	}

}



fn main() {
    println!("{}", foo());
}
