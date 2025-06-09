use crate::sound::instrument::clarinet;

mod sound{
    pub mod instrument{
        pub fn clarinet(){
            println!("este es un clarinete")
        }

        pub fn trombone(){
            println!("este es un trombon")
        }
    }

    pub mod stringed_instrument{
        fn guitar(){
            println!("este es un isntrumento de cuerdo") //private
        }
    }
}

fn main() {
    //ruta absoluta
    crate::sound::instrument::clarinet();

    //ruta relativa
    sound::instrument::trombone();
}
