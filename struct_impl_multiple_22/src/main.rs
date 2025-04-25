/*
 se puede separar los metodos en distintas impl pero no es tan comun
 solo en algunos casos que veremos mas adelante
 */

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    
    let r = Rectangle{
        width: 2,
        height: 2
    };

    let r1 = Rectangle{
        width: 1,
        height: 1
    };

    println!("area: {}, ¿Sí encaja ? {}", r.area(), r.can_hold(&r1));

}
