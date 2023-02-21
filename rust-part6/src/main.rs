fn main() {
    let _array = [1, 2, 3];
    let _vector1 = vec![1, 2, 3, 4, 5, 6];
    let mut _vector2: Vec<i32> = Vec::new();
    _vector2.push(1);
    _vector2.push(2);
    _vector2.push(3);
    _vector2.push(4);
    println!("vector 2 = {:?}", _vector2);
    
    let four = &_vector1[3];
    println!("{}",four);
}
