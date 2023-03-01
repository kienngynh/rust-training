use crossbeam_channel::unbounded;
use std::thread;

enum Worker {
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}
enum MainMsg {
    ResultSum(i32),
    MainQuit,
}
fn main() {
    let (main_tx, main_rx) = unbounded();
    let (sub_tx, sub_rx) = unbounded();
    let _thread_1 = thread::spawn(move || loop {
        match sub_rx.recv() {
            Ok(msg) => match msg {
                Worker::PrintMsg(data) => println!("{}", data),
                Worker::Sum(a, b) => {
                    println!("Worker summing");
                    match main_tx.send(MainMsg::ResultSum(a + b)) {
                        Ok(v) => v,
                        _ => (),
                    };
                }
                Worker::Quit => {
                    println!("Worker quite");
                    match main_tx.send(MainMsg::MainQuit) {
                        Ok(v) => v,
                        _ => (),
                    };
                    break;
                }
            },
            Err(e) => {
                println!("Error Quit {:?}", e);
                match main_tx.try_send(MainMsg::MainQuit) {
                    Ok(v) => v,
                    _ => (),
                };
                break;
            }
        }
    });
    /*         let _thread_2 = thread::spawn(move || loop { match main_rx.recv() {
            Ok(msg) => match msg {
                MainMsg::ResultSum(sum) => {
                    println!("Main sum: {}", sum);
                    break;
                }
                MainMsg::MainQuit => {
                    println!("Main quite");
                    break;
                }
            },
            Err(e) => {
                println!("Error main quite: {:?}", e);
                break;
            }
        }});
    */

    match sub_tx.send(Worker::PrintMsg(String::from("My name is Kien"))) {
        Ok(v) => v,
        _ => (),
    };
    match sub_tx.send(Worker::Sum(5, 17)) {
        Ok(v) => v,
        _ => (),
    };
    match sub_tx.send(Worker::Quit) {
        Ok(v) => v,
        _ => (),
    };   
    //drop(sub_tx);
    while let Ok(msg) = main_rx.recv() {
        match msg{
            MainMsg::ResultSum(a) => println!("Main result = {}",a),
            MainMsg::MainQuit => println!("Main quite"),
        }
    }
    match _thread_1.join() {
        Ok(()) => (),
        _ => (),
    };
}
