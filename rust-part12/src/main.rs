#[allow(dead_code)]
fn caps(input: &str) -> String {
    input.to_uppercase()
}
#[cfg(test)]
mod test {
    use crate::caps;
    #[test]
    fn check() {
        let result = caps("nguyen huu kien");
        let expected = String::from("NGUYEN HUU KIEN");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}
fn math(a: i32, b: i32, ob: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    ob(a, b)
}
fn main() {
    let data: Vec<_> = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();
    for num in data {
        println!("{}", num);
    }

    let add = |a: i32, b: i32| a + b;
    let mult = |a: i32, b: i32| a * b;
    println!("{} {}", add(2, 6), mult(3, 8));
    println!("{}", math(2,3,Box::new(|a, b| a + b)));
    println!("{}", math(7,8,Box::new(|a, b| a * b)));
}
