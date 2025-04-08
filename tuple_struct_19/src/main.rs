fn main() {

    /*
    ejemplo de tupla normal
     */

    let normal = (10, "hola", 1);
    println!("normal {}, {}, {}", normal.0, normal.1, normal.2);

    //des-estructurar
    let (x, y, z) = normal;
    println!("norma des-estructurar: {} {} {}", x, y, z);

    /*
    ejemplo de tuple struct
     */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let init = Point(0, 0, 0);

    println!("color blanco {}, {}, {}", black.0, black.1, black.2);
    println!("punto inical {}, {}, {}", init.0, init.1, init.2);

    //des-estructurar
    let Color(a, b, c) = black;
    println!("back des-estructurar: {} {} {}", a, b, c);


    /*
    ejemplo de tuple struct unit-like (sin nungun campo)
     */

    struct Unit();
    let u : Unit;
}
