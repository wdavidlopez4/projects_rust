
/*
    primera version de programa para calcular rectangulo
    sin utilizar estructuras
 */
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("el area es {} en pixeles", area(width1, height1)); 
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
