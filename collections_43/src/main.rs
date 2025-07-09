fn main() {

    /*
        vectores (listas)
    */

    // Declaración de un vector vacío
    let vector : Vec<i32> = Vec::new();
    println!("{:?}", vector);

    // Declaración de un vector con valores iniciales, "vec!" es una macro
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    //añadiendo elementos al vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);


    // Accediendo a los elementos del vector
    let v = vec![1, 2, 3, 4, 5];
    println!("El primer elemento es: {}", v[0]);

    match v.get(2) {
        Some(value) => println!("El tercer elemento es: {}", value),
        None => println!("No hay un tercer elemento."),
    }

    //recorriendo un vector
    let vector1 = vec![1, 2, 3, 4, 5 ];
    for i in &vector1{
        println!("El elemento es: {}", i);
    }

    //recorriendo un vector mutable
    let mut vector2 = vec![1, 2, 3, 4];
    for v in &mut vector2{
        *v += 50; // Modificando el valor del elemento (el * es para desreferenciar la variable mutable)
        println!("El elemento modificado es: {}", v);
    }

    println!("Vector modificado: {:?}", vector2);


    //ejemplo de vector con enum para utilizar diferentes tipos de datos
    #[derive(Debug)]
    enum SheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SheetCell::Int(3),
        SheetCell::Float(3.14),
        SheetCell::Text(String::from("12"))
    ];

    println!("Fila: {:?}", row[0]);
}
