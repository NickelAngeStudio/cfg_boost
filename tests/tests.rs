extern crate target_cfg;
use target_cfg::{ target_cfg, cfg_target };

#[doc(hidden)]
#[cfg_target(x86_64:ar & desktop | (mobile))] 
/// Blabla
/// Bdkd dskd
fn test_fnt() {
    println!("foo1");
}

target_cfg!{
    
    
    (linux:os & wasm:fm) => {
		/// Cmt l1
        /// l2
        /// l3
        /// ```
        /// let a = 16;
        /// ```
        pub fn inside()  {
            println!("inside me");

            if true {
                if !false {
                    println!("Wow, this is useless!");
                }
            }
        }

        /// Cmt ll1
        /// ll2
        /// ll3
        /// ```
        /// let ab = 136;
        /// ```
        pub fn inside_me_2()  {
            println!("inside me");

            if true {
                if !false {
                    println!("Wow, this is useless!");
                }
            }
        }
        
	},
	() => {
	
	},
    
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