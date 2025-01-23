fn main() {

    //variable mutable
    let mut x = 5;
    println!("variable mutable: {}", x);

    x = 6;
    println!("variable mutable: {}", x);

    //variable inmutable
    let a = 4;
    println!("variable inmutable: {}", a);

    //constante
    const MAX_PAINTS: u32 = 100_000;
    println!("constante: {}", MAX_PAINTS);

    //variable sombreada (cambia el valor a una variable mutable, pero sigue siendo mutable)
    let j = 5;
    println!("primer valor de variable sombreada {}", j);

    let j = 6;
    let j = j + 1;

    println!("la variable sombreada es: {} ", j);

    //variable sombreada (puede cambiar de tipo y reutilizar el mismo nombre)
    let spaces = "   ";
    let spaces = spaces.len();
    println!("variable sombreada que cambia de tipo de valor: {}", spaces);

}
