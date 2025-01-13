use std::io;

fn main() {
    println!("!Adivina el numero¡");
    println!("Introdusca su adivinanza.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("fallo al leer la linea");

    println!("Tu numero: {}", guess);
}
