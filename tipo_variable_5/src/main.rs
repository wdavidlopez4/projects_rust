fn main() {
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

    //ejemplo variable caracter
    let c = 'Z'; //en comillas simples
    println!("caracter: {}", c);
}
