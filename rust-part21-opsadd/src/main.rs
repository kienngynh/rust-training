use std::ops::{Index,Add};
struct Letter(String);
impl Add<Self> for Letter {
    type Output = String;
    fn add(self, input: Self) -> Self::Output {
        format!("{}{}", self.0, input.0)
    }
}
#[derive(Debug)]
struct Number(i32);
impl Add<i32> for Number {
/*     type Output = i32;
    fn add(self,input:i32) -> Self::Output{
        self.0 + input
    }
  */
    type Output = Self;
    fn add(self,input:i32) -> Self::Output{
        Number(self.0 + input)
    }
}
#[derive(Debug)]
#[allow(dead_code)]
enum Vinfast {
    Vf6,
    Vf7,
    Vf8,
    Vf9
}
#[derive(Debug)]
struct Price{
    price_vf6:i64,
    price_vf7:i64,
    price_vf8:i64,
    price_vf9:i64,
}
impl Index<&Vinfast> for Price{
    type Output = i64;
    fn index(&self,brand:&Vinfast) -> &Self::Output{
        match brand {
            Vinfast::Vf6 => &self.price_vf6,
            Vinfast::Vf7 => &self.price_vf7,
            Vinfast::Vf8 => &self.price_vf8,
            Vinfast::Vf9 => &self.price_vf9,
        }
    }
}
fn main() {
    println!("{}",Letter("h".to_owned()).add(Letter("i".to_owned())));
    println!("{}",Letter("h".to_owned()) + Letter("ello".to_owned()));
    println!("{:?}",Number(15)+5);
    let price = Price{
        price_vf6:40000,
        price_vf7:45000,
        price_vf8:48000,
        price_vf9:52000,
    };
    let vf = Vinfast::Vf8;
    println!("price of {:?}: {}",vf,price.index(&vf))
}
