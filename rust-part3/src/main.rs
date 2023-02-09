#[derive(Debug)]
struct Member {
    _user_name: String,
    _email: String,
    _age: u64,
    _active: bool,
}
struct Hinhchunhat{
    dai:u32,
    rong:u32,
}
impl Hinhchunhat {
    fn dien_tich(&self) -> u32{
        self.dai * self.rong
    }
    fn chua(&self,hinhchunhatkhac:&Hinhchunhat) -> bool{
        self.dai > hinhchunhatkhac.dai && self.rong > hinhchunhatkhac.rong
    }
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

    let _kich_thuoc = Hinhchunhat{dai:60,rong:30};
    println!("Dien tich hinh chu nhat la {}", _kich_thuoc.dien_tich());
    let _kich_thuoc2 = Hinhchunhat{dai:50,rong:20};
    let _kich_thuoc3 = Hinhchunhat{dai:70,rong:20};
    println!("Hinh chu nhat co the chua hinh 2: {}", _kich_thuoc.chua(&_kich_thuoc2));
    println!("Hinh chu nhat co the chua hinh 3: {}", _kich_thuoc.chua(&_kich_thuoc3));
}

fn create_new_member(_user_name: String, _email: String, _age: u64) -> Member {
    Member {
        _user_name: _user_name,
        _email: _email,
        _age: _age,
        _active: true,
    }
}
