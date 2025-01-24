fn main() {

    //variable mutable
    let mut x = 5;
    println!("variable mutable: {}", x);

    x = 6;
    println!("variable mutable: {}", x);

    //variable inmutable
    let a = 4;
    println!("variable inmutable: {}", a);

    //otro ejemplo de variable mutable (siempre va a ser mutable)
    let mut h = 2;
    h = h +1;
    println!("otro ejemplo de variable mutable :{}", h ); 

    //constante
    const MAX_PAINTS: u32 = 100_000;
    println!("constante: {}", MAX_PAINTS);

    //variable sombreada (cambia el valor a una variable inmutable, pero sigue siendo inmutable despues de cambiar el valor)
    let j = 5;
    println!("primer valor de variable sombreada {}", j);

    let j = 6;
    let j = j + 1;

    println!("la variable sombreada es: {} ", j);

    //variable sombreada (puede cambiar de tipo y reutilizar el mismo nombre) (siempre va a ser inmutable)
    let spaces = "   ";
    let spaces = spaces.len();
    println!("variable sombreada que cambia de tipo de valor: {}", spaces);


    //ejemplo variable entera
    let con_signo : u32 = 12121;
    let sin_signo : i32 =  -112;

    println!("con signo: {}, sin signo: {}", con_signo, sin_signo);

    //ejemplo variable decimal
    let coma_flotante : f32 = 32.122;
    println!("coma flotante: {}", coma_flotante);

    //ejemplo de operacion aritmetica
    let resto = 43 % 5;
    let divicion = 43 /5;

    println!("divicion: {}, resto: {}", divicion, resto);

    //ejemplo variable boleana
    let variable_boleana = true;
    let variable_true: bool = true;

    println!("variable bolena: {}, notacion explixita: {}", variable_boleana, variable_true);

}
