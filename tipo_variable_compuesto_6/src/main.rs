fn main() {
    /*
    valores compuestos
    */

    //ejemplo tupla
    let tupla: (i32, f64, u8) = (500, 6.3, 1); //su tamaño es fijo
    let (x, y, z) = tupla; //des-estructurar

    println!("tupla: {} {} {}", x, y, z);

    //otra forma de escribir una tupla
    let otra_tupla = (4, 3, 1.1);
    let value_final = otra_tupla.2;

    println!("otra tupla: {}", value_final);


    //ejemplo array
    let array_ejemplo = [1, 2, 3, 4]; //siempre el mismo valor y longitud no cambia
    let elemento_array = array_ejemplo[2];

    println!("elemento array: {}", elemento_array);

    //otra forma de definir un array
    let otro_array: [i32; 4] = [1, 2, 3, 4];
    println!("elemento array: {}", otro_array[0]);
}
