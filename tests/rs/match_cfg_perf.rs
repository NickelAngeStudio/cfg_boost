#![cfg_attr(docsrs, feature(doc_cfg))]

use cfg_boost::match_cfg;

pub fn inner_cfg() {

    let a = match_cfg!{
        desktop => 10,
        mobile => 20,
        _ => 30,
    };

    println!("{}", a);

}

fn main() {
    inner_cfg();
}
