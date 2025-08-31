
/*
esta funcion quiere decir:
X y Y tienen un tiempo de vida generico 'a
donde 'a es el tiempo de vida mas corto de X 0 Y 
es decir, 'a toma el tiempo de vida mas pequeño que posee estos parametros 
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        return x;
    }
    else{
        return y;
    }
}

fn main() {

    /* 
    ejemplo donde funciona 
    */
    let string1 = "hola"; //tiempo de vida mas largo por que es un slice
    let string2 = String::from("perro"); //tiempo de vida mas corto por que es un string

    let result =longest(string1, &string2);

    println!("Hello, world! {}", result);



    /*
    ejemplo donde no funciona 
    no compila por que el tiempo de vida de string4 tiene solo el alcance en {}
    
    el compilador coge el tiempo de vida mas corto que es el string4 y determina que 
    no puede dar un resultado un valor

    aunque, el valor string3 tenga mas grande el tiempo de vida solo es escoge el tiempo
    de vida mas corto string4
    */
    let string3 = String::from("perro");

    {
        let string4 = String::from("gato");
        
    }

    let result1 =longest(&string3, &string4);
    
    println!("Hello, world! {}", result1);




    /*
    ejemplo refactor donde si funciona 
    */
    let string3 = String::from("perro");

    {
        let string4 = String::from("gato");
        let result1 =longest(&string3, &string4);
        println!("Hello, world! {}", result1);
    }
    
    
}
