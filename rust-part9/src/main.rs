fn main() {
    let num1 = 10;
    let num2 = 20;
    println!("{}", get_ref(&num1, &num2));
}
fn get_ref<'a>(parameter1: &'a i32, parameter2: &'a i32) -> &'a i32 {
    if parameter1 > parameter2 {
        parameter1
    } else {
        parameter2
    }
}
