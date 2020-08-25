/* Try modifying Cacher to hold a hash map rather than a single value.
   The keys of the hash map will be the arg values that are passed in,
   and the values of the hash map will be the result of calling the closure
   on that key. Instead of looking at whether self.value directly has a
   Some or a None value, the value function will look up the arg in the hash
   map and return the value if it’s present. If it’s not present, the Cacher
   will call the closure and save the resulting value in the hash map
   associated with its arg value.
*/

/*  This task was significantly more difficult that I suspected it would be.

    https://stackoverflow.com/questions/53331382/how-do-i-build-a-cacher-in-rust-without-relying-on-the-copy-trait
    This my initial thoughts:
    https://gist.github.com/anonymous/2cab56f3263ca0706e6ee5bfb60d3e2a
    Way better:
    https://gist.github.com/m1el/4583d806cc58640b34565fe0218e7551
    Probably ideal:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=3b9f2f67ce57a22583d68511295b2ecb
*/ 
use std::collections::HashMap;

struct Cacher<T>
where T: Fn(u32) -> u32,
{
    calculation: T,
    // Make this the hashmap?
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // For loop,match, functions??
        match self.value {
            // instead of this search the hashmap
            Some(v) => v,
            None => {
                // Instead of this push to the hashmap
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn main() {
    println!("Sorry - Ive not implemented a solution to this - hopefully I revisit soon");
}

#[test]
fn call_with_different_values(){
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_ne!(v1, v2);
}
