
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 5,
        height: 4
    };

    /*
        es importante saber que rect1 es una referencia de Rectangle por lo tanto,
     no puede llamar directamente a los metodos de Rectangle, pero rust lo hace por nosobre 
     y por debajo permite que una instancia pueda invocar un metodo como vemos en este ejemplo

     en otros lenguajes como c/c++ se puede llamar desde una refencia al meto utilizando el operador (->) en ves del punto (.)
     pero rust lo hace automaticamente por nosotros
     */
    let value_area: u32 = rect1.area();

    println!("el area es {}", value_area);
}
