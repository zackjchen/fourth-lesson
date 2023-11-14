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

    // 第一种方法
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

    // 第二种方法
    let dog = Dog{name: "旺财".into()};
    let cat = Cat{name: "咪咪".into()};
    let bird = Bird{name: "小明".into()};

    let mut arr2:Vec<&dyn Animal> = vec![&dog,&cat, &bird];
    for animal in arr2 {
        animal.action()
    }

    // 区别：
    // 第一种方法的数据拥有所有权，第二种需要引用，存放的应该是指针，不是引用的话大小不确定   
}


trait Animal {
    fn action(&self);
}

struct Cat{
    name: String,
}

struct Dog {
    name: String,
}

struct Bird{
    name: String,
}
impl Animal for Cat {
    fn action(&self) {
        println!("猫猫{:?}抓老鼠",self.name)
    }

}

impl Animal for Dog {
    fn action(&self) {
        println!("狗狗{:?}会看家",self.name)
    }

}

impl Animal for Bird {
    fn action(&self) {
        println!("这只鸟儿不会飞")
    }
}

