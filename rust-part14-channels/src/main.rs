use crossbeam_channel::unbounded;
use std::thread;
fn main() {
    let (sender, receiver) = unbounded();
    let _thread_1 = thread::spawn(move || match receiver.recv() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e),
    });
    match sender.send("Hello, I am Nguyen Huu Kien!") {
        Ok(v) => v,
        _ => (),
    };
    match _thread_1.join() {
        Ok(v) => v,
        _ => (),
    };
}
