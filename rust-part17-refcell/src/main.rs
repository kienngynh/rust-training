use ::std::cell::RefCell;

struct Channel {
    name: RefCell<String>,
}

fn main() {
    let mychannel = Channel {
        name: RefCell::new(String::from("Lap trinh Blockchain cung Kien")),
    };
    match mychannel.name.try_borrow() {
        Ok(v) => println!("mychannel: {:?}", v),
        Err(e) => println!("{:?}", e),
    };
    {
    let mut a = mychannel.name.borrow_mut();
    *a = String::from("Lap Trinh Blockchain cung Kien")
    };
    {
    mychannel
        .name
        .replace(String::from("Lap trinh blockchain cung Vu"))
    };
    match mychannel.name.try_borrow() {
        Ok(v) => println!("mychannel: {:?}", v),
        Err(e) => println!("{:?}", e),
    };
}
