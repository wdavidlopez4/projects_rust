mod maths;

use maths::statistic::Statistic;


fn main() {
    /*
    crear los datos
     */
    let mut list_number: Vec<i32> = vec![8, 3, 4, 5, 4, 2, 2, 11, 9, 12, 1, 3, 3];

    let mut statistic = Statistic::new();

    /*
    calcular promedio
     */
    
    statistic.set_average(&list_number);

    println!("este es el promedio: {}", statistic.get_average());

    /*
    ordenar datos y calcular mediana
     */

    let ordered_list_number = 
        Statistic::bubble_sort(& mut list_number);

    statistic.set_median(ordered_list_number);

    println!("datos: ordenada {:?}, mediana: {}", ordered_list_number, statistic.get_median());


    /*
    calcular la moda
     */

    let map = Statistic::create_map(&list_number);
    statistic.set_mode(&map);

    let mode = statistic.get_mode();

    println!("{:?}, y la moda es {} con {} ", map, mode.get_key(), mode.get_value());

}
