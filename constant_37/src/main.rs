
//es cxonstante pero mutable, no es seguro modificarla, es mejor no modificarla
static LANGUAGE : &str = "Rust";

//constante comun
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    //imprimiendo
    println!("el lenguaje es: {}", LANGUAGE);
    println!("el limite es: {}", THRESHOLD);

    //ejemplo error
    //THRESHOLD = 5;

    //ejemplo constante statica
    let es = 
    if is_big(n){
        "big"
    }
    else{
        "small"
    };

    println!("es {}", es);
}
