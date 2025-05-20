use std::mem;

fn main() {

    /*
    arrays
     */

    //ejemplo array
    let xs: [i32; 5] = [1, 2, 3, 4, 5 ];

    //array que dice que va a tener 500 elementos con el mismo valor 1
    let ys: [i32; 500] = [1; 500];

    //acceder a los valores
    println!("{}", xs[0]);
    println!("{}", ys[480]);

    //obtener tamaño 
    println!("{}", xs.len());

    //podemos saber cuantos bytes ocupa un arreglo (i32 es 32 bits)
    println!("el arreglo ocupa: {} bytes", mem::size_of_val(&xs));


    /*
    slices
     */

    //crear un slice
    //recuerde que un slice se crear apartir de un array o un vector
    let array: [i32; 4] = [1, 2, 3, 4 ];
    let slice_complet: &[i32] = &array[..];
    let slice_part: &[i32] = &array[1..3];
    println!("tamanos de slices: {}, {}", slice_complet.len(), slice_part.len());

    //tomamos un arreglo le lo comvertimos en slice es decir en un tipo dinamico
    analyze_slice(&xs);

    //toma una porcion del arreglo y lo convierte en slice
    analyze_slice(&ys[1 .. 3]);

    // lanza un error porque se sale de la longitud
    //println!("{}", xs[5]);
}

//esta funcion toma prestada los valores del array y se concierte en un slice
fn analyze_slice(slice: &[i32]){
    println!("primer elemento del slice: {}", slice[0]);
    println!("tiene la cantidad de elemntos {}", slice.len());
}
