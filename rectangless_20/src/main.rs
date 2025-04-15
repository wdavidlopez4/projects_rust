struct Rectangle{
    width: u32,
    height: u32
}

fn main() {

    /*
        primera version de programa para calcular rectangulo
        sin utilizar estructuras
        no se relacionan las variables
    */
    let width1 = 30;
    let height1 = 50;

    println!("el area es {} en pixeles", area(width1, height1)); 

    /*
        segunda version de programa para calcular rectangulo
        utilizando tuplas
        el problema de esta version es que nos podemos confundir cual es el ancho o el alto
    */
    let rect1 = (30, 50);
    println!("el area es {} en pixeles con tupla", area_tuplas(rect1));


    /*
        tercer version de programa para calcular rectangulo
        utilizando estruturas
        mayor claridad

        recordar que se pasa un prestamo (referencia inmutable) al metodo area_struct
    */


    let rect2 = Rectangle{
        width: 30,
        height: 50
    };

    println!("el area es {} en pixeles con estructura", area_struct(&rect2));


}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuplas(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}
