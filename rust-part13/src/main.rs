use std::thread;
use std::time::Duration;

fn msg_hello() -> String {
    thread::sleep(Duration::from_secs(1));
    String::from("Hello, ")
}
fn msg_myname() -> String {
    thread::sleep(Duration::from_secs(2));
    String::from("My name is ")
}
fn msg_name() -> String {
    thread::sleep(Duration::from_secs(3));
    String::from("Nguyễn Hữu Kiên!")
}
fn main() {
    let msg_1 = thread::spawn(move || msg_hello());
    let msg_2 = thread::spawn(move || msg_myname());
    let msg_3 = thread::spawn(move || msg_name());

    let msg_1 = msg_1.join().expect("msg_1 error");
    let msg_2 = msg_2.join().expect("msg_2 error");
    let msg_3 = msg_3.join().expect("msg_3 error");
    println!("{}{}{}", msg_1, msg_2, msg_3);
}
