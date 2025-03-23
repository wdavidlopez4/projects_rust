fn main() {

    /*
        referencias inmutables
     */


    //ejemplo: le pasamos la referencia de la variable text a la funcion calculate_le...()
    let text = String::from("hola mundo");
    let len = calculate_length(&text);
    println!("{} - {}", text, len);

    //regla: podemos tener varias referencias inmutables en un mismo ambito
    let c = String::from("regla");
    let c1 = &c;
    let c2 = &c;
    println!("{}, {}", c1, c2);


    /*
        referencias mutables
     */

    //ejemplo: le pasamos una referencia mutable y cambiamos el valor origianl de la variable
    let mut text1 = String::from("hola");
    change(&mut text1);
    println!("cambio: {}", text1);

    //regla: solo se puede tener una referencias mutable a un dato en un ambito
    let mut s = String::from("ejemplo regla");
    let s1 = &mut s;
    //let s2 = &mut s;        error solo se puede tener una referencia mutable
    println!("{}", s1);

    //regla: podemos usar llaves para para crear nuevos ambitos,
    //permitiendo multiples referencias mutables
    //pero no simultaneas
    let mut a = String::from("hello");

    {
        let a1 = &mut a;
        println!("en otro contexto {}", a1);
    }

    {
        let a2 = &mut a;
        println!("en otro contexto {}", a2);
    }

    //regla: si las referencias son inmutables entonces no podemos tener una mutable
    let mut d = String::from("regla"); //no es necesario poner mut porque todas las referencias son inmutables
    let d1 = &d;
    let d2 = &d;
        //let d3 = &mut c; no pues existir referencia mutable
    println!("{}, {}", d1, d2);

    

}

//funcion que resibe una referencia
fn calculate_length(text: &String) -> usize{
    text.len()
}


/*
funcion que resibe una referencia mutable,
es decir, si es mutable cambia el valor original de la variable
 */
fn change(text: &mut String){
    text.push_str(", mundo");
}