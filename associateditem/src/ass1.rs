trait Contains<A,B> {
     fn contains(&self, _:&A, _:&B) -> bool;
     fn first(&self) -> i32;
     fn last(&self) -> i32;
}

struct Container(i32,i32);
impl Contains<i32,i32> for Container {
     fn contains(&self, number_1:&i32, number_2:&i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
     }

     fn first(&self) -> i32{
        self.0
     }

     fn last(&self) -> i32 {
        self.1
     }
}

#[derive(Debug)]
struct Container2 {
    param_1:i32,
    param_2:i32,
}
impl Contains<i32,i32> for Container2 {
    fn contains(&self, number_1:&i32, number_2:&i32) -> bool {
        (&self.param_1 == number_1) && (&self.param_2 == number_2)
     }

     fn first(&self) -> i32{
        self.param_1
     }

     fn last(&self) -> i32 {
        self.param_2
     }
}

fn difference<A,B,C,D,E,F>(container:&C, container_2:&F) -> (i32,i32) where
    C:Contains<A,B>, F:Contains<D,E>{
    
    (container.first() - container.last(), container_2.first() - container_2.last())
}

pub fn ass1(){
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    let numberb_1 = 3;
    let numberb_2 = 10;

    let container2 = Container2{
        param_1:numberb_1,
        param_2:numberb_2,
    };

    println!("Does container contain {} and {}: {}",
        &numberb_1, &numberb_2,
        container2.contains(&numberb_1, &numberb_2));
    println!("First number: {}", container2.first());
    println!("Last number: {}", container2.last());

    println!("The difference is: {:?}", difference(&container,&container2));
    
}