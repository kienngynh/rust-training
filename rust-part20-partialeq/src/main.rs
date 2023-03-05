use std::cmp::{Ordering, PartialOrd};
#[derive(PartialEq, PartialOrd)]
enum Employer {
    Marketer(i32),
    Developer(i32),
}
#[derive(PartialEq)]
struct User {
    id: i32,
    name: String,
}
impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        /*         if self.name < other.name {
                  Some(Ordering::Less)
              } else if self.name > other.name {
                  Some(Ordering::Greater)
              } else {
                  Some(Ordering::Equal)
              }
        */
        Some(self.name.cmp(&other.name))
    }
}
fn main() {
    let a = Employer::Marketer(50);
    let b = Employer::Developer(40);
    // with Enum, if condition compare index
    if a > b {
        println!("true")
    } else {
        println!("false")
    }
    let c = User {
        id: 1,
        name: "AB".to_owned(),
    };
    let d = User {
        id: 2,
        name: "B".to_owned(),
    };
    // with Struct, partialEq,PartialOrd compare value of first key
    if c < d {
        println!("true")
    } else {
        println!("false")
    }
    println!("{:?}", c.partial_cmp(&d));
}
