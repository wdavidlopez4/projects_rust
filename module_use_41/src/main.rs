
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

fn main() {
    guitar::play();
    piano::play();
}
