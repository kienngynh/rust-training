#[derive(Debug)]
struct Point<T, Y> {
    x: T,
    y: Y,
    z: T,
}
impl<T, Y> Point<T, Y> {
    fn mixed<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: (self.x),
            y: (other.y),
            z: (self.z),
        }
    }
}
fn main() {
    let _point1 = Point {
        x: 4,
        y: 10.2,
        z: 3,
    };
    let _point2 = Point { x: 5, y: 'v', z: 6 };
    let _point3 = _point1.mixed(_point2);
    println!("{:?}", _point3);
    //println!("{} {}", _point1.x + _point1.z, _point1.y);
    let _number_list = vec![20, 32, 2024, 45, 235, 123, 353];
    let mut _max_number = &_number_list[0];
    for _number in &_number_list {
        if _number > _max_number {
            _max_number = _number;
        }
    }
    println!(
        " The max number is {}",
        get_max_number(_number_list.clone())
    );
    println!(" The max number is {}", get_max(_number_list.clone()));
    let _character_list = vec!['c', 'q', 'd', 'e', 'b'];
    println!(
        " The max character is {}",
        get_max_character(_character_list.clone())
    );
    println!(" The max character is {}", get_max(_character_list.clone()));
}

fn get_max_number(_number_list: Vec<i32>) -> i32 {
    let mut _max_number = _number_list[0];
    for _number in _number_list {
        if _number > _max_number {
            _max_number = _number;
        }
    }
    _max_number
}

fn get_max_character(_character_list: Vec<char>) -> char {
    let mut _max_character = _character_list[0];
    for _character in _character_list {
        if _character > _max_character {
            _max_character = _character;
        }
    }
    _max_character
}

fn get_max<T: PartialOrd + Copy>(_list: Vec<T>) -> T {
    let mut _max = _list[0];
    for _i in _list {
        if _i > _max {
            _max = _i;
        }
    }
    _max
}
