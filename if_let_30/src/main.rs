/*
el if let solo se debe usar para comparar un pero no muchos valores para ello
utilizar match
*/

enum State_Quarter{
    One,
    Two
}

enum Coin{
    Quarter(State_Quarter)
}

fn main() {
    /*
    ejemplo 1
     */

    let some_u8 = Some(3u8);

    if let Some(3) = some_u8 {
        println!("es tres")
    }


    /*
    ejemplo 2
     */

    let coin = Coin::Quarter(State_Quarter::One);

    if let Coin::Quarter(State_Quarter::Two) = coin {
        println!("es two")
    }
    else {
        println!("es one")
    }


}
