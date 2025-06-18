
mod instruments{
    pub mod guitar {
        pub fn play(){
            println!("Playing guitar!");
        }
    }

    pub mod piano {
        pub fn play(){
            println!("Playing piano!");
        }
    }
}

use crate::instruments::guitar; //absolute path
use self::instruments::piano; //relative path (puede que cambie en el futoru es una inconsistencia de rust poner self)
use std::fmt::Result;
use std::io::Result as IoResult; //renombrar Result a IoResult

fn main() {
    guitar::play();
    piano::play();

    _ =  example_io_result();
    _ = example_result();
}

// Ejemplo de como se llama a un elemento renombrado
fn example_io_result() -> IoResult<()>{
    Ok(())
}

fn example_result() -> Result{
    Ok(())
}
