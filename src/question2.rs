use std::fmt::Debug;
use std::ops::{Add, Deref};
use std::process::Output;

#[derive(Debug,Clone,Default)]
struct Tuple2{
    var1: i32,
    var2: i32,
}

impl Add for Tuple2 {
    type Output = Tuple2;
    fn add(self, rhs: Self) -> Tuple2 {
        let var1 = self.var1 + rhs.var1;
        let var2 = self.var2 + rhs.var2;
        Self::Output{var1, var2}
    }
}
#[derive(Debug,Clone,Default)]
struct Tuple3{
    var1: i32,
    var2: i32,
    var3: i32
}

impl Add for Tuple3 {
    type Output = Tuple3;
    fn add(self, rhs: Self) -> Tuple3 {
        let var1 = self.var1 + rhs.var1;
        let var2 = self.var2 + rhs.var2;
        let var3 = self.var3 + rhs.var3;
        Self::Output{var1, var2, var3}
    }
}


fn main() {
    let a = Tuple2{var1:1,var2:4};
    // let b = Tuple2{var1:2,var2:5};
    let res = func(a);
    println!("{:?}",res)
}

fn func<T>(data: Box<dyn Add<T,Output:T>>){
    // 想知道怎么获取trait对象的数据类型
    println!("{:?}",data)
}
