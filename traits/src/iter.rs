#![feature(is_sorted)]
#[derive(Debug,PartialEq,PartialOrd)]
struct DoIter {
    input : Vec<(String,i32)>
}

use std::any::type_name;

impl DoIter {
     fn iter_to_map(&self){
        let new_val = &self.input
        .iter()
        .map(|(_name,_val)| {
            _name
        })
        .collect::<Vec<_>>();

        println!("{:?}",new_val);
     }

     fn iter_to_map_sort(&self){
        let mut new_val = self.input
        .iter().map(|(_name,_val)| {
            _val
        })
        .collect::<Vec<_>>();

        new_val.sort();
        let el = check_index(new_val, 1);

        println!("{:?}",el.unwrap());
     }
}

pub fn call_iter(){
    let call_iter = DoIter{
        input:vec![("Jannes".to_string(),3),("Santoso".to_string(),2)]
    };
    call_iter.iter_to_map();
    call_iter.iter_to_map_sort();
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn check_index(v: Vec<&i32>, idx: usize) -> Option<&i32>{
    if idx > v.len()-1 {
        None
    } else {
        Some(v[idx])
    }
}