extern crate target_cfg;
use target_cfg::{ target_cfg, cfg_target };

#[cfg_target([@] x86_64:ar & linux:os)]
pub fn cfg_target_test() {
    println!("Is available");
}

pub fn inside3()  { 
    println!("iside me");
    target_cfg!{ [!$, @, #]
        ([!*] linux:os) => {
                let a = 15;
                if true {
                    if !false {
                        println!("Wow, this is uselessin Linux!");
                    }
                }

               
            },
        
        ([!*] windows:os) => {
                if true {
                    if !false {
                        println!("Wow, this is useless in Windows!");
                    }
                }
        },
        
        ([@, !*]) => {
            println!("invalid node?");
        },
        _ => {
            println!("last branch node?");
        },
        
    }

}

target_cfg!{
    
    
    (linux:os | (wasm:fm | windows:os)) => {

        struct example {
            myvar : u32,

            pub myvar2 : u64,
        }

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
	(x86:ar) => {
        pub fn last() {

        }
	},
    
}



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