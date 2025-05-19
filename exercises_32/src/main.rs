#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    /*
    ejemplo de tupla en una funcion
     */
    reverse((2, false));


    /*
    ejemplo operaciones tuplas
     */

    // Una tupla con un montón de tipos diferentes
    let long_tuple = (
        1u8, 2u8, 3u8, 4u8, 
        -1i8, -2i16, -3i16, -3i16,
        0.1f32, 0.2f64, 'a', true);

    // Los valores se pueden extraer de la tupla mediante la indexación de tuplas
    println!("Long tuple first value {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Las tuplas pueden ser miembros de tuplas
    let tuple_of_tuples = (
        (1u8, 2u8, 2u32), 
        (4u64, -1i8), 
        -2i16 );

    //las tuplas hijas se imprimen
    println!("tuple of tuples: {:?}", tuple_of_tuples.0);

    //pero si la tupla es muy larga lanza un error que no se puede imprimir
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);

    // Para crear tuplas de un solo elemento, 
    //se requiere la coma para distinguirlas de un literal rodeado de paréntesis.
    //un solo elemento de una tupla
    println!("one element tuple: {:?}", (5u32,));

    //Las tuplas se pueden desestructurar para crear enlaces
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    /*
    ejemplo estructura tupla
     */
    let matrix = Matrix(1.1, 1.2, 1.3, 1.4); //se tipa
    print!("{:?}", matrix);

}


fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (integer_value, boolen_value) = pair;

    (boolen_value, integer_value)
}
