fn main() {
    let _array = [1, 2, 3];
    let _vector1 = vec![1, 2, 3, 4, 5, 6];
    let mut _vector2: Vec<i32> = Vec::new();
    _vector2.push(1);
    _vector2.push(2);
    _vector2.push(3);
    _vector2.push(4);
    println!("vector 2 = {:?}", _vector2);

    let _four = &_vector1[4];
    match _vector1.get(3){
        Some(_four)=>println!("This is four element = {}",_four),
        None => println!("This is not four element"),
    }
}
