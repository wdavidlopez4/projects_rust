use std::collections::HashMap;

struct Mode{
    value : i32,
    key: i32
}

impl Mode {
    fn new() -> Self{
        Mode{
            value: 0,
            key: 0
        }
    }

    fn set(&mut self, key: i32, value: i32){
        self.key = key;
        self.value = value;
    }

    fn get_key(&self) -> &i32 {
        &self.key
    }

    fn get_value(&self) -> &i32 {
        &self.value
    }
}


struct Statistic{
    average : f32, //promedio
    median: f32, //media,
    mode: Mode //moda
}

impl Statistic{
    fn new() -> Self{
        Statistic{ 
            average: 0.0, 
            median: 0.0, 
            mode: Mode::new()
        }
    }

    fn set_average(&mut self, list_number: &Vec<i32>){
        let mut sum: i32 = 0;
        let len = list_number.len() as f32;

        for number in list_number{
            sum = sum + number;
        }

        self.average = sum as f32 / len;
    }

    fn get_average(&mut self) -> f32{
        self.average
    }

    fn bubble_sort(lis_number: & mut Vec<i32>) -> &Vec<i32>{
        let len = lis_number.len();

        for i in 0..len -1 { //para recorrer a todos los numeros recordar que va hasta el indice -1
            let mut swapped = false;
            
            for j in 0..len - 1 - i{ //para comparar un numero con todos (menos el indice) (menos el valor del 1 ciclo, 2 ciclo...)
                
                if lis_number[j] > lis_number[j + 1] { //hacemos el intercambio
                    
                    (lis_number[j], lis_number[j + 1]) = (lis_number[j +1], lis_number[j]);
                    swapped = true;

                }
            }

            if ! swapped { break }
        }

        lis_number
    }

    fn set_median(&mut self, ordered_list_number: &Vec<i32>){
        let count = ordered_list_number.len();
        println!("count: {}", count);
        if count % 2 == 0 {
            let posicion = (count - 1) / 2;

            let p2 = ordered_list_number[posicion] as f32;
            let p1 = ordered_list_number[posicion + 1] as f32;
            
            self.median = (p1 + p2) / 2 as f32;
        }
        else{
            println!("entro impar");
            let posicion = (count) / 2;
            self.median = ordered_list_number[posicion] as f32;
        };
    }

    fn get_median(&mut self) -> f32{
        self.median
    }

    fn create_map(list_number: &Vec<i32>) -> HashMap<i32, i32>{
        let mut map: HashMap<i32, i32> = HashMap::new();

        for number in list_number{
            match map.get(number) {
                Some(value) => {
                    map.insert(*number, *value + 1);
                },
                None => {
                    map.insert(*number, 1);
                }
            }
        }

        map
    }

    fn set_mode(&mut self, map: &HashMap<i32, i32>){
        let mut max_value = 0;
        let mut max_key = 0;

        for (key, value) in map {
            if *value > max_value{
                max_value = *value;
                max_key = *key;
            }
        }

        self.mode.set(max_key, max_value);
    }

    fn get_mode(&mut self) -> &Mode{
        &self.mode
    }
}

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
