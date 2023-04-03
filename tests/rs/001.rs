// Test 001 : MissingOperator error for target_cfg! macro.
use cfg_boost::target_cfg;

target_cfg!{
	linux windows => {
		fn foo() {
			panic!("Should not be called!");
		}
	},
}

fn main() {
    foo();
}
