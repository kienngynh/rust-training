use backoff::ExponentialBackoff;
use parking_lot::{Mutex, ReentrantMutex};
use std::thread;
use std::{rc::Rc, sync::Arc};

#[allow(dead_code)]
type Data = Rc<ReentrantMutex<u32>>;
#[allow(dead_code)]
fn recurse(data: Data, id: u32) -> u32 {
    match id {
        id if id == 0 => 0,
        id => recurse(Rc::clone(&data), id + 1),
    }
}
type ArcAccount = Arc<Mutex<Account>>;
fn arcaccount(account: Account) -> ArcAccount {
    Arc::new(Mutex::new(account))
}
struct Account {
    balance: usize,
}
fn transfer(from: ArcAccount, to: ArcAccount, amount: usize) {
    let op = || {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                return Ok(())
            }
        }
        Err(0)?
    };
    let backoff = ExponentialBackoff::default();
    match backoff::retry(backoff, op){
        Ok(v) => v,
        Err(e) => println!("{:?}",e),
    };
}
fn main() {
    let a = arcaccount(Account { balance: 500 });
    let b = arcaccount(Account { balance: 600 });
    let transaction_1 = thread::spawn(move || transfer(a, b, 100));
    match transaction_1.join() {
        Ok(v) => println!("{:?}",v),
        Err(e) => println!("{:?}",e)
    };
}
