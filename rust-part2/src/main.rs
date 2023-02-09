fn main() {
    // Stack and Heap
    fn _x() {
        let _a = "Hello";
        let _b = 100;
        _y();
    }
    fn _y() {
        let _a: &str = "World";
    }

    // Rust rule 1: mỗi giá trị trong rust chỉ có 1 biến được gọi là ownership
    let _s1 = gives_ownership();
    let (mut _s1, _len1) = calculate_lenght_method_1(_s1);
    let _len2 = calculate_lenght_method_2(&mut _s1);
    println!("length method 1 = {}, length method 2 = {}", _len1, _len2);
    let _s2 = _s1.clone();
    let _s3 = takes_and_gives_ownership(_s2.clone());
    takes_ownership(_s1.clone());
    println!("s1 = {}, s2 = {}, s3 = {}", _s1, _s2, _s3);

    fn takes_ownership(_some_string: String) {
        println!("some string = {}", _some_string)
    }

    fn gives_ownership() -> String {
        let _some_string = String::from("Hello World!");
        _some_string
    }

    fn takes_and_gives_ownership(_some_string: String) -> String {
        _some_string
    }

    fn calculate_lenght_method_1(_some_string: String) -> (String, usize) {
        let _length = _some_string.len();
        (_some_string, _length)
    }
    fn calculate_lenght_method_2(_some_string: &mut String) -> usize {
        _some_string.push_str("!");
        let _length = _some_string.len();
        _length
    }

    // Rust rule 2: chỉ có 1 owner tại 1 thời điểm (chỉ tham chiếu được tới 1 mut duy nhất tại 1 thời điểm)
    let _a = String::from("Hi hi");
    let _a1 = &_a;
    let _a2 = &_a;
    println!("{} {}", _a1, _a2);
}
