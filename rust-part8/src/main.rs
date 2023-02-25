#[derive(Debug)]
struct Data2 {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}
struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}
impl Data {
    fn new() -> Self {
        Data {
            num1: (15),
            num2: (25),
            str1: (String::from("Some string 2")),
            optional: (None),
        }
    }
}
trait Transform {
    fn revert(&self) -> String{
        // Init funtion
        String::from("No string ...")
    }
    fn output_revert(&self) {
        println!("{}",self.revert());
    }
}
impl Transform for Data {
    fn revert(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}
impl Transform for Data2 {
    fn revert(&self) -> String {
        (self.num1 + self.num2).to_string()
    }
}
fn main() {
    let a: Data = Data {
        num1: 10,
        num2: 20,
        str1: String::from("Some string"),
        optional: Some(5),
    };
    a.output_revert();
    println!("{:?} {:?} {:?} {:?}", a.num1, a.num2, a.str1, a.optional);

    let b = Data::new();
    println!("{:?}", b.revert());

    let c = Data2 {
        num1: 26,
        num2: 42,
        str1: String::from("ok string"),
        optional: Some(24),
    };
    println!("{:?} {:?} {:?} {:?}", c.num1, c.num2, c.str1, c.optional);
    println!("{}", c.revert());
}
