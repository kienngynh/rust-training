#[derive(Debug)]
enum IpAddressKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
struct _IpAddress {
    kind: IpAddressKind,
    address: String,
}
impl _IpAddress{
    fn some_funtion (){
        println!("Blockchain Development");
    }
}
fn main() {

    let localhost = IpAddressKind::V4(127,0,0,1);
    println!("local host = {:#?}",localhost);
}
