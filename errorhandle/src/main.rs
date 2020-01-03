use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    handle_error1();
    let result = read_username_from_file();
    println!("{:?}",result);
    let res2 = simple_read_username_from_file();
    let d = res2.unwrap();
    println!("{:?}",d);
    let res3 = return_all_type_error();
    println!("{:?}",res3);
}

fn handle_error1(){
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}",error);
            })
        } else {
            panic!("Problem opening file {:?}",error);
        }
    });

    println!("{:?}",f);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn simple_read_username_from_file() -> Result<String, io::Error> {
    let mut f  = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn return_all_type_error() -> Result<(), io::Error> {
    Ok(())
}