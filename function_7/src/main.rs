fn main() {
    //ejemplo de anotacion de funcion
    another_function();

    //ejemplo de parametros de una funcion
    another_function_param(3);

    //ejemplo con varios parametros
    another_function_params(2, 3);

    /*
    Las declaraciones son instrucciones 
    que realizan alguna acción y no devuelven ningún valor.

    Las expresiones devuelven algún valor.
     */


    //ejemplo de declaracion
    let y = 7; //porque declaramos, pero no retornamos valor, esto NO se puede let x = (let y = 6);
    println!("valor de declaracion {}", y);

    /*
    Las expresiones pueden formar parte de declaraciones
     */

    //ejemplo exp:  5 + 6 retorna 11
    //ejemplo exp:  el 6 de la declaración let y = 6; (6 retorna 6)
    //ejemplo exp:  llamar a una macro es un expresion
    //ejemplo exp:  llamar a una funcion
    //ejemplo exp:  El bloque que usamos para crear nuevos ámbitos, {}, es una expresión, 

    //ejemplo de exprecion  por {}
    let a = {
        let x = 3;
        x + 1 //sin ; para que retorne valor, ultima instruccion se retorna
    };

    println!("valor de expresion: {}", a);

}

fn another_function(){
    println!("asi se escribe un funcion");
}

fn another_function_param(x: i32){
    println!("funcion con parametro: {}", x);
}

fn another_function_params(x: i32, y: i32){
    println!("funcion con varios parametros: {}, {}", x, y);
}
