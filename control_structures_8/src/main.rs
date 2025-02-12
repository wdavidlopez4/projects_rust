
fn main() {
    control_if();
    control_else_if();
    control_expression();
}

fn control_if(){
    let number = 3;

    if number < 5{
        println!("es verdadero");
    }
    else {
        println!("valor falso")
    }
}

fn control_else_if(){
    let number = 6;

    if number % 4 == 0{
        println!("numero es divisible por 4");
    }
    else if number % 3 == 0{
        println!("numero es divisible por 3");
    }
    else if number & 2 == 0{
        println!("numero es divisible por 2");
    }
    else{
        println!("no es divisible");
    }
}

//el if es una expresion (que retorna el valor) (el valor de retorno debe ser del mismo tipo)
fn control_expression(){
    let condition = true;

    let number = if condition{
        5
    }
    else{
        4 
    };

    println!("el valor de la expresion es: {}", number);
}
