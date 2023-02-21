use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let _array = [1, 2, 3];
    let mut _vector1 = vec![0, 1, 2, 3, 4, 5, 6];
    let mut _vector2: Vec<i32> = Vec::new();
    _vector2.push(1);
    _vector2.push(2);
    _vector2.push(3);
    _vector2.push(4);
    println!("vector 2 = {:?}", _vector2);

    match _vector1.get(3) {
        Some(_v) => println!("This is four element = {}", _v),
        None => println!("This is not four element"),
    }
    for i in &mut _vector1 {
        *i += 10;
    }
    println!("{:?}", _vector1);

    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SheetCell::Int(5),
        SheetCell::Float(12.15),
        SheetCell::Text(String::from("Blue")),
    ];

    match &row[0] {
        &SheetCell::Float(i) => println!("{}", i),
        _ => println!("this is not a float"),
    }

    let _vn = String::from("Tôi là Nguyễn Hữu Kiên");
    for i in _vn.bytes() {
        println!("{}", i);
    }
    for i in _vn.chars() {
        println!(" {}", i);
    }
    for i in _vn.graphemes(true) {
        print!(" {}", i);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("MU"), 10);
    scores.insert(String::from("MC"), 9);
    scores.entry(String::from("MU")).or_insert(5);

    println!("MU = {:?}, MC = {:?}",scores.get(&String::from("MU")),scores.get(&String::from("MC")));

    for (k,v) in scores{
        println!("key = {}, value = {}",k,v);
    }

    let text = "This world is the wonderful world";
    let mut map = HashMap::new();
    for i in text.split_whitespace(){
       *map.entry(i).or_insert(0) +=1;
    }
    println!("{:?}",map);
}
