use target_cfg::cfg_target;

#[cfg_target()]
fn cfg_empty() {
	println!("001");
}

fn main() {
    cfg_empty();
}
