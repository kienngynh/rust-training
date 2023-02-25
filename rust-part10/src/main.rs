// Authentication
// Authorization
struct Employee {
    position: Position,
    status: Status,
}
#[derive(Debug)]
enum Position {
    IT,
    CEO,
    CTO,
    Manager,
    Marketer,
}
enum Status {
    Active,
    Denied,
}
fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Denied => return Err("Access denied".to_owned()),
        _ => (),
    }
    match employee.position {
        Position::CEO => Ok(()),
        Position::CTO => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}
fn print_access(employee: &Employee) -> Result<(), String> {
    try_access(employee)?;
    println!("{:?} allow access", employee.position);
    Ok(())
}

fn main() {
    let manager = Employee {
        position: (Position::Manager),
        status: (Status::Active),
    };
    let it = Employee {
        position: (Position::IT),
        status: (Status::Active),
    };
    let ceo = Employee {
        position: (Position::CEO),
        status: (Status::Active),
    };
    let cto = Employee {
        position: (Position::CTO),
        status: (Status::Denied),
    };
    let marketer = Employee {
        position: (Position::Marketer),
        status: (Status::Denied),
    };
    print_access(&manager);
    print_access(&it);
    print_access(&ceo);
    print_access(&cto);
    print_access(&marketer);
}
