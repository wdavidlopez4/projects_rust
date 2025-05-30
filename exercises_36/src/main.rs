#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two
}

enum Color{
    Red = 0xffff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

fn main() {
    //imprime los numeros 
    println!("zero: {}, one: {}", 
        Number::Zero as i32, 
        Number::One as i32);


    //es rozado por que solo imprime los ultimos 6 digitos de red FF0000 
    println!("roses as #{:06x}", Color::Red as i32);

    //imprime los colores
    println!("Green as #{:x}", Color::Green as i32);
    println!("Green as #{:x}", Color::Blue as i32);
}
