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
}
