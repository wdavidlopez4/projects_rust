#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
}

fn value_in_cents(coin: Coin) -> u32{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("el estado del quarter es {:?}", state);
            25
        }
    }
}
