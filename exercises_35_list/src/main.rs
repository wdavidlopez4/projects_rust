
/*
    ejerccicio de lista
 */
use List::*;


//enumerador con const que representa el constructor del enumerador
//tiene un datos u32
//esta apuntando al siquiente nodo Box<List>
//Box es un puntero inteligente que deja el dato en header no en pila
//Nill es el ultimo nodo vacio
enum List{
    Constructor(u32, Box<List>),
    Nil
}

impl List{
    //es una funcion
    fn new() -> List{
        Nil
    }

    //metodos
    fn prepent(self, elem: u32) -> List {
        Constructor(elem, Box::new(self))
    }

    fn len(&self) -> u32{
        match *self {
            Constructor(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String{
        match *self {
            Constructor(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    //crear lista 
    let mut list = List::new();

    //antemoner elemento
    list = list.prepent(1);
    list = list.prepent(2);
    list = list.prepent(3);

    //mostrar
    println!("tamano {}", list.len());
    println!("elementos: {}", list.stringify());
}
