extern crate target_cfg;
use target_cfg::{ target_cfg, cfg_target };

#[cfg_target(x86_64:ar & linux:os)]
pub fn cfg_target_test() {
    println!("Is available");
}

/*
fn aaas() {
    mod tt {
        const a:u8 = 16;

        struct ddd {

        }
    }

    struct gg {

    }
}


#[doc(hidden)]
#[cfg_target(x86_64:ar & desktop | (mobile))] 
/// Blabla
/// Bdkd dskd
fn test_fnt() {
    println!("foo1");

    target_cfg!{ 
        (linux:os) => {
            
            if true {
                if !false {
                    println!("Wow, this is uslessin Linux!");
                }
            }

            fn inner_me() {
                println!("ssss");
            }
            inner_me();
        },
        
        (windows:os) => {
            if true {
                if !false {
                    println!("Wow, this is useless in Windows!");
                }
            }
        },
    }
}

target_cfg!{
    
    
    (linux:os | (wasm:fm | windows:os)) => {

        mod toto {

        }

        const MY_CONST:u32 = 65;

		/// Cmt l1
        /// l2
        /// l3
        /// ```
        /// let a = 16;
        /// ```
        pub fn inside()  {
            println!("iside me");
            target_cfg!{
                (linux:os) => {
                    
                        if true {
                            if !false {
                                println!("Wow, this is uselessin Linux!");
                            }
                        }

                       
                    },
                
                (windows:os) => {
                        if true {
                            if !false {
                                println!("Wow, this is useless in Windows!");
                            }
                        }
                },
            }

        }

        /// Cmt ll12
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
        pub fn last() {

        }
	},
    
}

*/

#[cfg(test)]
mod tests {

    use std::env;

    use crate::cfg_target_test;


        #[test]
        fn test() {

            
            //for (key, value) in std::env::vars() {
            //    println!("{key}: {value}");
           // }

            cfg_target_test();
            
        }
        
    

}