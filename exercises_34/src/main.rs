/*
estructura con referencias
 */

#[derive(Debug)]
struct Person<'a>{ //lifetime (tiempo de vida) persone. name va a vivir tanto viva la referencia que se pase
    name: &'a str,
    age: u8,
}

/*
estructura unit para los genericos
 */
struct None;

/*
estructura tipo (struct tuple)
 */
struct Pair(i32, f32);

/*
estructura con 2 propiedades
 */
struct Poin{
    x: f32,
    y: f32
}

/*
estructura con propiedades de estructura
    nos permite tener la estructura sin utilizar, 
    si no le ponemos el #[allow(dead_code)]  
    entonces nos molestara con la advertencia de que no lo estamos utilizando
 */
#[allow(dead_code)] 
struct Rectangle {
    p1: Poin,
    p2: Poin
}

fn main() {
    //crear una estructura que se le pasa referencia
    let name = "peter";
    let age = 27;

    let peter = Person
    {
        name: name,
        age: age
    };

    println!("persona {:?}", peter);


    //instanciar la estructura tipo puntos
    let pont : Poin = Poin 
    { 
        x: 0.33, 
        y: 0.4 
    };

    println!("acceder a los puntos {} - {}", pont.x, pont.y);


    //crear otro punto con el mismo valor de pont
    let new_pont = Poin
    {
        x: 0.1,
        ..pont //el resto de valores igual que pont
    };

    println!("acceder a los puntos del nuevo point: {} - {}", new_pont.x, new_pont.y);


    //desestructurando una estructura con let binding
    let Poin {x: my_x, y: my_y} = pont;
    println!("desestructurando: {} - {}", my_x, my_y);


    //crear una estructura con los point
    let _rectangle = Rectangle
    {
        p1: Poin { x: my_x, y: my_y },
        p2: pont
    };

    //instanciar un unit
    let none = None;

    //instanciar una tuple structur
    let pair = Pair(1, 0.1);
    println!("tupla structur {} - {}", pair.0, pair.1);

    //desestructurando
    let Pair(integer_fielt, decimal_fielt) = pair;
    println!("pair contains {:?} and {:?}", integer_fielt, decimal_fielt);

}
