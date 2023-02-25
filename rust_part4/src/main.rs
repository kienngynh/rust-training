fn main() {
    let _asset = decimals(Coin::Bitcoin(Balance::Shark));
    println!("asset = {}", _asset);

    let _five = Some(5);
    let _six = plus_one(_five);
    println!("six = {:#?}",_six);
    
    let _value = Some(4);
    match _value{
        Some(5) => println!("bang 5"),
        _ => println!("khac 5"),

    }
}
#[derive(Debug)]
enum Balance {
    //Small,
    //Intermediate,
    //Fish,
    Shark,
}
enum Coin {
    //Solana,
    //Ethereum,
   // Near,
    Bitcoin(Balance),
}
fn decimals(coin: Coin) -> u8 {
    match coin {
        //Coin::Solana => 1,
        //Coin::Ethereum => 10,
        //Coin::Near => 20,
        Coin::Bitcoin(bala) => {
            println!("I am a {:#?}", bala);
            30
        }
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x{
        Some(x) => Some(x+1),
        _ => None,
    }
}
