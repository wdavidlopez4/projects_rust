use std::panic;


fn main() {
    control_if();
    control_else_if();
    control_if_expression();
    control_loop();
    control_loop_expression();
    control_while();

    let result = panic::catch_unwind(||{
        control_while_otro_ejercicio();
    });

    if result.is_err() {
        println!("entro en panico la funcion control_while_otro_ejercicio");
    }

    control_for();
    control_for_cuenta_regresiva();
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
fn control_if_expression(){
    let condition = true;

    let number = if condition{
        5
    }
    else{
        4 
    };

    println!("el valor de la expresion es: {}", number);
}

fn control_loop(){
    let mut count = 0;

    loop{
        if count > 3{
            break;
        }

        println!("valor loop es: {}", count);
        count = count +1;
    }
}

fn control_loop_expression(){
    let mut count = 0;

    let result = loop{
        if count == 5 {
            break count * 2;
        }

        count += 1;
    };

    println!("valor loop expression es: {}", result);
}

//este ciclo reemplaza a loop cuendo utilizamos if, else, breack dentro del ciclo "es mas facil"
fn control_while(){
    let mut count = 4;

    while count != 0 {
        println!("el valor de while es {}", count);

        count = count - 1;
    }
}

fn control_while_otro_ejercicio(){
    let elemnets = [10, 4, 2, 1];
    let mut index = 0;

    while index < 4 {
        println!("elemento de arreglo {} ", elemnets[index]);
        index = index + 1;
    }

}

//mucho mejor que control_while_otro_ejercicio
fn control_for(){
    let elements = [3, 4, 3, 1];

    for element in elements.iter(){
        println!("elemento de arreglo version for {}", element);
    }
}

fn control_for_cuenta_regresiva(){ 
    for element in (1..4).rev() {
        println!("cuenta regresiva: {} !", element);
    }
}
