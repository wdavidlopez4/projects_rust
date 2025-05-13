fn main() {

    /*
        ejemplo en el cual se debe cumpliar todas las ramas del match (match con option<T>)
     */
    //pasar option con valor 5
    let five = Some(5);
    let six = plus_one(five);
    println!("valor es: {}", six.unwrap());

    //pasar node (nada)
    let none = plus_one(None);
    println!("es none: {}", none.is_none());


    /*
        ejemplo en el cual se debe cumplir unicamente las ramas seleccionadas (match con numero natural)
     */

    //no hace nada 
    plus_one_2(0); 

    //ejecuta
    plus_one_2(5);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("no pasa nada retornamos none");
            None
        },
        Some(i) => Some(i + 1)
    }
}

fn plus_one_2(x: u8){
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => () // si no cumple con las anteriores ramas entonces no hace nada
    }
}
