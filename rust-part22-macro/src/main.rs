#[derive(Debug)]
#[allow(dead_code)]
enum ClassType {
    Good,
    Bad,
}
#[derive(Debug)]
#[allow(dead_code)]
struct Class {
    name: String,
    kind: ClassType,
}
fn main() {
    /* let a = 1;
       let b = 1;
       assert_ne!(a, b, "value should not be equal");
    */
    /* let classA = Class {
        name: "A15".to_owned(),
        kind: ClassType::Good,
    };
    dbg!(&classA); */

    let a = "Hello";
       let b = "World";
       println!("{}",format!("{} {}",a,b));
   
}
