extern crate rand;

use std::io;
use rand::Rng; //trait
use std::cmp::Ordering; //enumerador

fn main() {
    println!("!Adivina el numero¡");

    let secret_number = rand::thread_rng()
        .gen_range(1.. 101);

    println!("El numero secreto es: {}", secret_number);

    println!("Introdusca su adivinanza.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("fallo al leer la linea");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("escriba un numero");

    let comparison_result = guess.cmp(&secret_number); //comparamos quess y el secret_number

    match comparison_result {
        Ordering::Less => println!("numero pequeño"),
        Ordering::Greater => println!("numero grande"),
        Ordering::Equal => println!("numero igual")
    };

    println!("Tu numero: {}", guess);
}
