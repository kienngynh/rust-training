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
#[allow(dead_code)]
fn test_1(param1: Vec<f64>) -> Vec<f64> {
    param1
} // Lifetime don't apply
#[allow(dead_code)]
fn test_2<'a>(param1: &'a Vec<f64>) -> &Vec<f64> {
    param1
} // Lifetime don't apply
#[allow(dead_code)]
fn test_3(param1: &Vec<f64>) -> &Vec<f64> {
    &param1
} // Lifetime don't apply
#[allow(dead_code)]
fn test_4<'a, 'b: 'a>(param1: i32, param2: &'a str, param3: &'b str, param4: i32) -> &'a str {
    if param1 == 7 && param4 > 10 {
        param2
    } else {
        param3
    }
}
