// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Test 034 : Auto documentation config true.
use cfg_boost::{ target_cfg, meta_cfg };


target_cfg!{
	zeus:os => {
		/// Function to call zeus in Zeus os
		pub fn call_zeus(){
			println!("Zeus has been called!");
		}
		
		/// Function to smite dirty peasant.
		pub fn smite_dirty_peasant(){
			println!("Dirty peasant has been smited!");
		}
	},
	hades:os => {
		/// Soul collecting function.
		pub fn suatms() {
			println!("Shut up and take my soul!");
		}
	},
	!doc & desktop => {
		/// Never documented function
		pub fn this_function_is_never_documented(){
			println!("Nobody write about me!");
		}
	}
}

/// Oh my god!
#[meta_cfg(apollo:os)]
pub fn omg() {

}

/// They don't talk about me!
#[meta_cfg(!doc & desktop)]
pub fn bruno() {
    println!("No no no no no!");
}


fn main() {
    this_function_is_never_documented();
    bruno();
}
