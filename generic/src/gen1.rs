pub fn gen1(){
    let p = Point{x:5,y:2};
    let x = p.x() + p.y();
    println!("generic 1 {}",x);
}


struct Point<T> {
    x:T,
    y:T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}