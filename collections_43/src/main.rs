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



    /*
    cadenas de caracteres o Strings
     */
    // Declaración de una cadena de caracteres vacía
    let cadena : String = String::new();
    println!("{:?}", cadena);

    // Declaración de una cadena de caracteres con valores iniciales
    let data: &str = "Hola, mundo!";
    let cadena: String = data.to_string();
    println!("{:?}", cadena);

    // otra forma de declarar una cadena de caracteres
    let cadena = String::from("!Hola, mundo!");
    println!("{:?}", cadena);

    //añadiendo caracteres a la cadena
    let mut s = String::from("Hola");
    let s2 = "mundo";
    s.push_str(s2); // se puede añadir una cadena completa


    s.push('!'); // solo se puede añadir un caracter a la vez
    println!("{}", s);

    //concatenando cadenas
    let s1 = String::from("Hola");
    let s2 = String::from(" mundo");
    let s3 = s1 + &s2; // s1 ya no es válido después de esta operación
    println!("{}", s3);


    //forma sensilla de concatenar cadenas
    let s1 = String::from("Hola");
    let s2 = String::from(" mundo");
    let s3 = format!("{}{}", s1, s2);
    println!("{}", s3);

    //iterar sobre una cadena
    println!("Iterando sobre la cadena:");
    let cadena = String::from("Hola, mundo!");
    for c in cadena.chars(){
        println!("{}", c); // Imprime cada caracter de la cadena con metodo chars()
    }

    println!("Iterando sobre los bytes de la cadena:");
    for c in cadena.bytes(){
        println!("{}", c); // Imprime el valor en bytes de cada caracter
    }

    /*
        hash maps (mapas hash)
    */
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);


    //hash maps y ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    println!("{:?}", map);
    // field_name y field_value ya no son válidos aquí, porque sus valores fueron movidos al HashMap

}
