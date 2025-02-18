use std::io;

fn main() {
    let mut degree_fahrenhein = String::new();
    let mut exit = String::new();

    welcome();

    loop{
        degree_fahrenhein.clear(); //limpia la cadena
        exit.clear(); //reinicia a ""

        println!("Ingrese una temperatura");

        io::stdin()
            .read_line(&mut degree_fahrenhein)
            .expect("fallo al leer la temperatura");

        let degree_fahrenhein:i32 = match degree_fahrenhein.trim().parse() { //recordar que parce retorna un enum que puede ser ok o err "fijate el match"
            Ok(num) => num,
            Err(_) => continue //continua la siquiente iteracion 
        };

        let result = convert_celsius(degree_fahrenhein);
        println!("el valor {} fahrenhein es {} celcius", degree_fahrenhein ,result);

        println!("letra s para finalizar.");
        
        io::stdin()
            .read_line(&mut exit)
            .expect("error al obtener salida del programa");

        println!("salida: {}", exit);
        
        match exit.trim() == "s" {
            true => break,
            false => ()
        }
    }
    
}

fn convert_celsius(fahrenhein: i32) -> f32{
    const SUBTRACTION: i32 = 32;
    const MULTIPLICATION: i32 = 5;
    const DIVISION: i32 = 9;

    let celsius: f32 = ((fahrenhein - SUBTRACTION) * MULTIPLICATION) as f32 / DIVISION as f32;

    celsius //valor de retorno
}

fn welcome(){
    //r# evita caracteres de escape
    println!(r#"
      ██████╗ ██████╗ ██╗   ██╗███████╗███╗   ██╗██╗  ██╗
     ██╔════╝ ██╔══██╗██║   ██║██╔════╝████╗  ██║██║  ██║
     ██║  ███╗██████╔╝██║   ██║█████╗  ██╔██╗ ██║███████║
     ██║   ██║██╔═══╝ ██║   ██║██╔══╝  ██║╚██╗██║██╔══██║
     ╚██████╔╝██║     ╚██████╔╝███████╗██║ ╚████║██║  ██║
      ╚═════╝ ╚═╝      ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝
      
      Convertidor de Fahrenheit a Celsius
        "#);
}
