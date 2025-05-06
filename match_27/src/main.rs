enum Coin{
    Penny,
    Nickel,
    Dime,
    Guarter
}

fn main() {
    let coin = value_in_cents(Coin::Dime);
    println!("{}", coin);
}

fn value_in_cents(coin: Coin)-> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => {
            println!("este es dime");
            3
        },
        Coin::Guarter => 4
    }
}
