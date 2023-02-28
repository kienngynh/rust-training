use crossbeam_channel::unbounded;
use std::thread;

enum Message {
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}
fn main() {
    let (sender, receiver) = unbounded();
    let _thread_1 = thread::spawn(move || loop {
        match receiver.recv() {
            Ok(msg) => match msg {
                Message::PrintMsg(data) => println!("{}", data),
                Message::Sum(a, b) => println!("{} + {} = {}", a, b, a + b),
                Message::Quit => {
                    println!("Quit program");
                    break;
                }
            },
            Err(e) => {
                println!("Error data: {:?}", e);
                break;
            }
        }
    });
    #[allow(unused)]
    sender.send(Message::PrintMsg(String::from("My name is Kien")));
    sender.send(Message::Sum(5, 17));
    sender.send(Message::Quit);
    //drop(sender);
    _thread_1.join();
}
