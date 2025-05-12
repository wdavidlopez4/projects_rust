fn main() {

    //pasar option con valor 5
    let five = Some(5);
    let six = plus_one(five);
    println!("valor es: {}", six.unwrap());

    //pasar node (nada)
    let none = plus_one(None);
    println!("es none: {}", none.is_none());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("no pasa nada retornamos none");
            None
        },
        Some(i) => Some(i + 1)
    }
}
