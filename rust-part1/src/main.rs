fn main() {
    let _x: i64 = 1123_43214124_432;
    println!("x = {}", _x);
    let _x = "ten";
    println!("x = {}", _x);

    // Scalar type
    let _sum = 3 + 4;
    let _subtraction = 20.4-13.2;
    let _multiplication = 5.2*6.0;
    let _division = 30/5;
    let _remainder = 56%9;
    println!("sum = {}", _sum);
    println!("subtraction = {}", _subtraction);
    println!("multiplication = {}", _multiplication);
    println!("division = {}", _division);
    println!("remainder = {}", _remainder);

    // Shadowing
    let _outer = 10;
    {
        let _inner = 200;
        println!("inner = {}", _inner);
        let _outer = 300;
        println!("outer= {}", _outer);
    }
    print!("outer = {}", _outer)
}
