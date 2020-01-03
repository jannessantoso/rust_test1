use std::fmt::Debug;

#[derive(Debug)]
struct Rectangle {length:f64, width:f64}
#[derive(Debug)]
struct Triangle {length:f64, width:f64}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {self.length * self.width}
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {self.length * self.width}
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}",t);
}

pub fn gen2() {
    let rectangle = Rectangle { length: 3.0, width: 4.0 };
    let triangle = Triangle  { length: 2.0, width: 3.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    print_debug(&triangle);
    println!("Area: {}", area(&triangle));
}