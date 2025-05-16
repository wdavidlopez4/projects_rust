fn main() {

    /*
    ejercicios de literales
     */

     //enteros
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    //boleanos
    println!("true AND false is {}", true && false);
    println!("true Or false is {}", true || false);
    println!("Not true is {}", !true);

    //bytes se puede utilizar 0x, 0o o 0b.
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); //dos formas de expresar un byte con u32 al final
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);

    //desplazamiento de bits a la izquierda o derecha
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //hexadecimal

    //facilitar la escrtura de numeros grandes
    println!("Escritura de millones {}", 1_000_000u32);
}
