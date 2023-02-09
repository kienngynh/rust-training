#[derive(Debug)]
struct Member {
    _user_name: String,
    _email: String,
    _age: u64,
    _active: bool,
}
fn main() {
    let mut _member1 = Member {
        _user_name: String::from("tulip_bk"),
        _email: String::from("kien.ngynh@gmail.com"),
        _age: 26,
        _active: true,
    };
    println!("Name 's member 1 is {}", _member1._user_name);
    _member1._user_name = String::from("kienngynh");
    println!("New name 's member 1 is {}", _member1._user_name);
    let _member2 = create_new_member(
        String::from("ori"),
        String::from("hoatulip97bn@gmail.com"),
        27,
    );
    println!("Member 2 is {:#?}", _member2);
}

fn create_new_member(_user_name: String, _email: String, _age: u64) -> Member {
    Member {
        _user_name: _user_name,
        _email: _email,
        _age: _age,
        _active: true,
    }
}
