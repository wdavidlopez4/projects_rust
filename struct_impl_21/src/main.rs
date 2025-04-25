
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    //metodo
    fn area(&self) -> u32{
        self.width * self.height
    }

    //metodo
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //funcion asociada (se llama asi por que no utiliza el struct)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
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


    /*
    utilizar un metodo para mirar si un rectangulo encaja en otro
     */

    let rec_a = Rectangle{
        width: 5,
        height: 5
    };

    let rec_b = Rectangle{
        width: 2,
        height: 4
    };

    let is_can_hold = rec_a.can_hold(&rec_b);
    
    println!("{}", is_can_hold);

    /*
    utilizando una funcion asociada
    (se llama asi por que no utiliza el struct)
     */

    let sq = Rectangle::square(12);
    println!("este es un cuadrado: h: {}, w: {}", sq.height, sq.width);

}
