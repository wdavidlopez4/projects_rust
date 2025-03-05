fn main() {
    regla_uno();
    regla_dos();
    regla_tres();
    ejemplo_primitivos();
}

/*
    tipos complejos: estan en el monton
 */

//Cada valor en Rust tiene un único propietario.
fn regla_uno(){
    let s = String::from("Hola mundo"); //s es el unico propietario del String
    println!("{}", s); // se puede usar por que s es el propietario
}

//Solo puede haber un propietario a la vez.
fn regla_dos(){
    let s1 = String::from("Hola mundo"); //s1 es el propietario del string
    let s2 = s1; //la propiedad se trasfiere a s2, s1 deja de ser valido

    println!("{}", s2); //por eso funciona
    //println!("{}", s1); //por eso no funciona
}

//Cuando el propietario sale de su alcance, el valor se elimina automáticamente.
fn regla_tres(){
    let s = String::from("hola mundo"); //s es creado dentro del bloque
    println!("{}", s); //se puede usar
}// aqui ya no se puede usar


/*
    tipos primitivos: estan en la pila
 */

//para los primitivos como estan en la pila, trait Copy (se copian no se mueven)
fn ejemplo_primitivos(){
    let s1 = 99; 
    let s2 = s1; //se copia, no se mueve

    println!("{}", s2); // valido
    println!("{}", s1); // valido
}
