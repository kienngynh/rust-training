fn main() {
    let _array = [1, 2, 3];
    let mut _vector1 = vec![0,1, 2, 3, 4, 5, 6];
    let mut _vector2: Vec<i32> = Vec::new();
    _vector2.push(1);
    _vector2.push(2);
    _vector2.push(3);
    _vector2.push(4);
    println!("vector 2 = {:?}", _vector2);

    match _vector1.get(3){
        Some(_v)=>println!("This is four element = {}",_v),
        None => println!("This is not four element"),
    }
    for i in & mut _vector1{
        *i += 10;
    }
    println!("{:?}",_vector1);
}
