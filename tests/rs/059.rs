// Test 059 : Modifier + and - on target_cfg!
use cfg_boost::{ target_cfg };


target_cfg! {

	linux => {},
	+ #[cfg(any())] => {
		pub fn foo1() -> String {
			String::from("Test")
		}
	},
	- #[cfg(all())] => {
		pub fn foo1() -> String {
			String::from("")
		}
	},
	+ #[cfg(any())] => {
		pub fn foo2() -> String {
			String::from("059")
		}
	},
	- #[cfg(all())] => {
		pub fn foo2() -> String {
			String::from("")
		}
	},
	#[cfg(all())] => {
		pub fn foo3() -> String {
			String::from("completed")
		}
	}

}

fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
