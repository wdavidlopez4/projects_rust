extern crate rand;

use std::io;

//trait
use rand::Rng;

fn main() {
    println!("!Adivina el numero¡");

    let secret_number = rand::thread_rng()
        .gen_range(1.. 101);

    println!("El numero secreto es: {}", secret_number);

    println!("Introdusca su adivinanza.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("fallo al leer la linea");

    println!("Tu numero: {}", guess);
}
