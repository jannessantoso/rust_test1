mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist(){
            println!("{}","dang");
        }
    }
}

use crate::front_house::hosting::add_to_waitlist;
use std::collections::HashMap;

fn main() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
    let mut map = HashMap::new();
    map.insert(1, 2);

    let dt = map.get(&1).unwrap_or_else(|| &1);
    println!("{:?}",dt);

    let dt2 = map.get(&3).unwrap_or_else(|| &4);
    println!("{:?}",dt2);
}
