use std::rc::Rc;
#[derive(Debug)]
struct Car {
    number: String,
}
#[derive(Debug)]
struct Door {
    vehicle: Rc<Car>,
}
fn main() {
    let car = Rc::new(Car {
        number: "99C -12345".to_owned(),
    });
    let left_door = Door{
        vehicle: Rc::clone(&car)
    };
    let right_door = Door{
        vehicle: Rc::clone(&car)
    };
    drop(car);
    println!("left door number {:?}",left_door.vehicle.number);
    println!("right door number {:?}",right_door.vehicle.number);
}
