fn main() {
    let s1 = gives_ownership(); //gives_ownership da el valor a s1

    let s2 = String::from("hola mundo 1"); //s2 entra en el ambito

    // s2 se mueve a takes_and_gives_back que tambien se mueve su valor a retorno a s3
    let s3 = takes_and_gives_back(s2); 

    println!("{} {}", s1, s3); //aqui si funciona s1 y s3
    //println!("{}" s2 ); //pero no funciona s2

}//aqui s1 y s3 salen del ambito

//movera su valor de retorno a la funcion que lo llame
fn gives_ownership() -> String{
    let some_string = String::from("hola mundo 2"); //entra en el ambito
    some_string //se devuelve y se mueve fuera de esta funcion
}

//funcion que toma y devuelve un string
fn takes_and_gives_back(some_string: String) -> String { //entra en el ambito
    some_string //se devuelve y se mueve fuera de la funcion
}


