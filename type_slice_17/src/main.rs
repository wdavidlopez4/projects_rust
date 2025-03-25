fn main() {

    /*
        ejemplo sin slice

        first_word() no retorna la longitud, 
        pero si tiene un espacio entonces nos retorna la posicion del espacio 

        si nos damos cuenta S es la variables que contierne "hola mundo"
        la funcion first_word() resibe por referencia No mutable a S

        despues obtenemos a WORK que es no mutable y es la posicion o longiutud del valor S
        demonos cuenta que S ni WORK estan relacionados, es decir si cambia S: "hola mundo"
        -> "hola" nunca S estaria enterada de que la longitud cambio o se quito el espacio
     */

    let mut s = String::from("hola mundo");
    let work = first_word(&s);

    s.clear();

    println!("{} {}", work, s);


    /*
        como se utiliza slice
     */

    //sin incluir
    let a = String::from("hola mundo");
    let a_hello = &a[0..4]; //toma desde 0 hasta 3
    let a_mundo = &a[5..10]; //toma desde 5 hasta 9
    println!("{}, {}", a_hello, a_mundo);

    //incluir
    let b = String::from("hola mundo");
    let b_hello = &b[0..=3]; //incluye el ultimo numero
    let b_mundo = &b[5..=9]; //incluye el ultimo numero
    println!("{}, {}", b_hello, b_mundo);

    let c = String::from("hola mundo");
    let c_hello = &c[..4]; //si el primer valor es 0 no hace falta ponerlo
    let c_mundo = &c[5..]; // si el ultimo es el ultmo no hace falta ponerlo

    println!("{}, {}", c_hello, c_mundo);

    let d = String::from("hola mundo");
    println!("{}", &d[..]); //con solo .. incluimos toda la palabra


    /*
        ejemplo de first_word con slice

        explicacion ejemplo de error
        recordemos que E es mutable y que si se saca 
        una referencua inmutable de E despues no pueden existir mas referencias mutables
        demonos cuenta que la referencua inmutables es E_WORD y que e.clear trata 
        de cambiar algo mutable que tiene referencia inmutable
     */

    //ejemplo de error:
    let mut e = String::from("hola mundo"); //mutables
    let e_word = first_word_slice(&e); //inmutable
    // e.clear();      //  error: hay referencia inmutable que se esta utilizando
    println!("the first word is {}", e_word);

    //ejemplo correcto
    let mut j = String::from("hola mundo"); //mutable
    let j_word = first_word_slice(&j); //inmutable
    println!("the first word is {}", j_word);
    j.clear(); //como ya no hay mas referencias entonces se puede limpiar

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    return s.len();
}

fn first_word_slice(s: &String) -> & str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]; //retorna hasta llegar al espacio
        }
    } 

    &s[..] //retorna toda la palabra
}