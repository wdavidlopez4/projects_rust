extern crate rand;

use rand::Rng; //trait
use std::cmp::Ordering;
use std::io; //enumerador

fn main() {
    println!("!Adivina el numero¡");

    let secret_number = rand::thread_rng()
        .gen_range(1..101);

    loop {
        println!("El numero secreto es: {}", secret_number);

        println!("Introdusca su adivinanza.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fallo al leer la linea");

        let guess: u32 = match guess.trim().parse() { //recordar que parce retorna un result tipo enum con variante ok y error, con match hacemos "como switch"
                Ok(num) => num,
                Err(_) => continue
        };

        let comparison_result = guess
            .cmp(&secret_number); //comparamos quess y el secret_number

        match comparison_result {
            Ordering::Less => println!("numero pequeño"),
            Ordering::Greater => println!("numero grande"),
            Ordering::Equal => {
                println!("numero igual");
                break;
            }
        };

        println!("Tu numero: {}", guess);
    }
}


//recordar lo siquiente 
//  guess.trim().parse() retorna un result que es un tipo enum y que tiene el valor ok y error
//  match value {} es como si fuera un switch
//  entonces:
//  en una sola line ponesmo poner "let guess: u32 = match guess.trim().parse() { ok... err... }" recordar que match es el que tiene los {} no parse()

