extern crate target_cfg;
use target_cfg::cfg_target;

#[doc(hidden)]
#[cfg_target(x86_64:ar & desktop | (mobile))] 
/// Blabla
/// Bdkd dskd
fn test_fnt() {
    println!("foo1");
}


#[cfg(test)]
mod tests {

    use crate::test_fnt;


        #[test]
        fn test() {

            if cfg!(target_os = "linux") {
                println!("dDFDDFSFFDS****ata");
            }
            test_fnt();
        }
        
    

}