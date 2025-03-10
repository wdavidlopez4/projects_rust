fn main() {
    /*
        se movera
     */
    let s = String::from("hello"); //s entra en el ambito

    takes_ownership(s); //s se mueve a la funcion 
                                    //y por lo tanto aqui ya no es valido

    // println!("{}", s); no se puede hacer


    /*
        se copiara
     */
    let x = 5; //x entra en el ambito
    makes_copy(x); //x se copia 
                                //por lo tanto aqui se puede utilizar

    println!("{}", x); //si se puede hacer
}

//toma la propiedad
fn takes_ownership(some_string: String){ //some_string entra en el ambito
    println!("{} ", some_string);
} //aqui, some_string sale del ambito de aplicaion y se llama a DROP, se libera memoria

//copia el valor
fn makes_copy(some_integer: i32){ //some_integer entra en el ambito
    println!("{} ", some_integer);
} // aqui, some_integer sale del ambito, Nada especial ocurre
