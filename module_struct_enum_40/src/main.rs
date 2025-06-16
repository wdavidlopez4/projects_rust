/*
    ejemplo estructura con mod
    recordar que la estructura puede ser publica
    si la estructura es publica NO quiere desir que sus propiedades tambien lo son
 */

use crate::menu::Appetizer;

 mod plant{
    pub struct Vegetable{
        pub name: String,
        id: i32
    } 

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable{
            Vegetable{
                name: String::from(name),
                id: 1
            }
        }
    }
 }

/*
ejemplo de enum con mod
si un enumerado es publico sus atributos tambien son publicos
 */

mod menu{

    #[derive(Debug)]
    pub enum Appetizer{
        Soup(String),
        Salad
    }
}


fn main() {
    /*
    estructura
     */
    let mut frijol = crate::plant::Vegetable::new("frijol");
    frijol.name = String::from("frijol mejorado");
    //frijol.id = 1; no se puede acceder por que es privado
    println!("name {}", frijol.name);


    /*
    enum
     */
    let order1 = Appetizer::Salad;
    let order2 = Appetizer::Soup(String::from("ensalada dulce"));

    println!("Menu: {:?} - {:?}", order1, order2);
}
