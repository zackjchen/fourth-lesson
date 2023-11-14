use std::fmt::Debug;
use std::ops::Add;
use crate::Enum::{Int, Str, Struct};
#[derive(Debug)]
enum Enum{
    Int(i32),
    Str(String),
    Struct(Tuple)
}
#[derive(Debug,Clone)]
struct Tuple{
    var1: i32,
    var2: i32,
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Self::Output {
        let var1 = self.var1 + rhs.var1;
        let var2 = self.var2 + rhs.var2;
        Self::Output{var1, var2}
    }
}





fn main() {
    let mut arr = vec![];
    arr.push(Int(-2));
    arr.push(Str("Hello".into()));
    arr.push(Struct(Tuple{var1:10,var2:20}));

    for element in arr.iter_mut() {
        match element {
            Int(int) => { *int = int.pow(2)}
            Str(string) => {string.push_str(" world")}
            Struct(obj) => {
                *obj = obj.to_owned() + Tuple{var1:1, var2: 1}
            }
        }
    }
    println!("{:?}",arr);
}
