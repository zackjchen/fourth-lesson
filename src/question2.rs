use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug)]
struct Tuple2{
    val1: i32,
    val2: i32
}

trait Add2<Rhs = Self>{
    type Output;
    fn add(&self, rhs: Rhs)-> Self::Output;
}
impl Add2 for Tuple2{
    type Output = Tuple2;

    fn add(&self, rhs: Self) -> Self::Output {
        let val1 = self.val1+rhs.val1;
        let val2 = self.val2+rhs.val2;
        Self::Output{val1,val2}
    }
}

fn main() {
    let a = Tuple2{val1:1, val2:2};
    let b = Tuple2{val1:3, val2:4};

    // let adder: &dyn Add2<Tuple2,Output=Tuple2> = &a;
    // let adder: Box<dyn Add2<Tuple2,Output=Tuple2>> = Box::new(a);
    // let c = adder.add(b);

    let c = test::<Tuple2>(&a, b);
    println!("{:?}",c)
}

fn test<T:Debug>(a:&dyn Add2<T,Output=T>, b:T) -> T{
    let c = a.add(b);
    c
}

