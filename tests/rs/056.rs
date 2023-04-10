// Test 056 : Modifier - on target_cfg!
use cfg_boost::{ target_cfg };


target_cfg! {

	linux => {},
	- #[cfg(all())] => {
		pub fn foo1() -> String {
			String::from("")
		}
	},
	- #[cfg(all())] => {
		pub fn foo2() -> String {
			String::from("")
		}
	},
	- #[cfg(all())] => {
		pub fn foo3() -> String {
			String::from("")
		}
	}

}

pub fn foo1() -> String {
	String::from("Test")
}

pub fn foo2() -> String {
	String::from("056")
}

pub fn foo3() -> String {
	String::from("completed")
}

fn main() {
    println!("{} {} {}!", foo1(), foo2(), foo3());
}
