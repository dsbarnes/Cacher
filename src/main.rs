// Try modifying Cacher to hold a hash map rather than a single value.
// The keys of the hash map will be the arg values that are passed in,
// and the values of the hash map will be the result of calling the closure
// on that key. Instead of looking at whether self.value directly has a
// Some or a None value, the value function will look up the arg in the hash
// map and return the value if it’s present. If it’s not present, the Cacher
// will call the closure and save the resulting value in the hash map
// associated with its arg value.
use std::collections::HashMap;

struct Cacher<T>
where T: Fn(u32) -> u32,
{
    calculation: T,
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
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn main() {
    println!("In progress");
}

#[test]
fn call_with_different_values(){
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
