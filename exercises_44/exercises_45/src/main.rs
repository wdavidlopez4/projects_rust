use std::fmt::Display;

fn main() {
    //utilizamos numeros
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    //utilizamos letras
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    //utilizando el 2 ejemplo
    let p = Pair::new(2, 4);
    p.cmp_display();

}

/*
ejemplo 1 metodo generico con clausula where
 */
fn largest<T>(list: &[T]) -> T 
    where T: PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }

    largest
}

/*
implementacion generica
 */

 struct Pair<T>{
    x: T,
    y: T
 }

 impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self{x, y}
    }
 }

 impl <T: Display + PartialOrd> Pair<T> { //Solo resibe de tipos Display y partiaord tambien se puede utilizar el where
     fn cmp_display(&self){
        if self.x >= self.y{
            println!("The lasgert menber is x = {}", self.x)
        }
        else {
            println!("The lasgert menber is y = {}", self.y)
        }
     }
 }