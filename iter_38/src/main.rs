/*
tipos de iteradores
 */

fn main() {
    /*
    iterador que toma prestado y no muta los valores
     */

    let names_1 = vec!["Bob", "William", "Deivit"];

    for name in names_1.iter(){
        match name {
            &"William" => println!("Es William lopez"),
            _ => println!("Es otra persona, su nombre: {}", name)
        };
    }

    println!("es posible: {}", names_1[0]);


    /*
    iterador que toma el valor (consume la coleccion), 
    es decir saca los elementos de uno a uno hasta quedar vacia la coleccion
     */

    let names_2 = vec!["Bob", "William", "Deivit"];

    for name in names_2.into_iter(){
        match name {
            "Bob" => println!("Es Bob esponja"),
            _ => println!("Es otro personaje: {}", name)
        }
    }

    //println!("es nos posible por que ya se consumio la coleccion en la iteracion: {}", names_2[0]);


    /*
    iterador que toma prestado (referencia) cada elemento de la coleccion 
    pero se puede mutar (alterar los valores)
    cuando utilizamos * como por ejemplo *name es que accede o modifica el valor apuntado. no a la referencia
     */
    let mut names_3 = vec!["Bob", "Andrey", "Deivit"];

    for name in names_3.iter_mut(){
        match name {
            &mut "Bob" => { *name = "Andrfey_0"}
            _ => println!("Es otro personaje: {}", name)
        }
    }

    println!("es posible: {}", names_3[0]);


    /*
    la ultima forma de iterar es sin necesidad de utilizar iter 
    en este caso no monifica los valores solo se pasan las referencias
     */
    let names_4 = vec!["Bob", "Andrey", "Deivit"];

    for name in &names_4{
        match name {
            &"Bob" => println!("es Bob sponja "),
            _ => println!("es otro")
        }
    }

    println!("es posible: {}", names_4[0]);

}
