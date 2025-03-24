fn main() {

    /*
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
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    return s.len();
}