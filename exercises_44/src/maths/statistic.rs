
use crate::maths::mode::Mode; //no necesitamos delcara el mod por que estamos en el mismo directorio en el mismo modulo
use std::collections::HashMap;

pub struct Statistic {
    average: f32, //promedio
    median: f32,  //media,
    mode: Mode,   //moda
}

impl Statistic {
    pub fn new() -> Self {
        Statistic {
            average: 0.0,
            median: 0.0,
            mode: Mode::new(),
        }
    }

    pub fn set_average(&mut self, list_number: &Vec<i32>) {
        let mut sum: i32 = 0;
        let len = list_number.len() as f32;

        for number in list_number {
            sum = sum + number;
        }

        self.average = sum as f32 / len;
    }

    pub fn get_average(&mut self) -> f32 {
        self.average
    }

    pub fn bubble_sort(lis_number: &mut Vec<i32>) -> &Vec<i32> {
        let len = lis_number.len();

        for i in 0..len - 1 {
            //para recorrer a todos los numeros recordar que va hasta el indice -1
            let mut swapped = false;

            for j in 0..len - 1 - i {
                //para comparar un numero con todos (menos el indice) (menos el valor del 1 ciclo, 2 ciclo...)

                if lis_number[j] > lis_number[j + 1] {
                    //hacemos el intercambio

                    (lis_number[j], lis_number[j + 1]) = (lis_number[j + 1], lis_number[j]);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }

        lis_number
    }

    pub fn set_median(&mut self, ordered_list_number: &Vec<i32>) {
        let count = ordered_list_number.len();
        println!("count: {}", count);
        if count % 2 == 0 {
            let posicion = (count - 1) / 2;

            let p2 = ordered_list_number[posicion] as f32;
            let p1 = ordered_list_number[posicion + 1] as f32;

            self.median = (p1 + p2) / 2 as f32;
        } else {
            println!("entro impar");
            let posicion = (count) / 2;
            self.median = ordered_list_number[posicion] as f32;
        };
    }

    pub fn get_median(&mut self) -> f32 {
        self.median
    }

    pub fn create_map(list_number: &Vec<i32>) -> HashMap<i32, i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for number in list_number {
            match map.get(number) {
                Some(value) => {
                    map.insert(*number, *value + 1);
                }
                None => {
                    map.insert(*number, 1);
                }
            }
        }

        map
    }

    pub fn set_mode(&mut self, map: &HashMap<i32, i32>) {
        let mut max_value = 0;
        let mut max_key = 0;

        for (key, value) in map {
            if *value > max_value {
                max_value = *value;
                max_key = *key;
            }
        }

        self.mode.set(max_key, max_value);
    }

    pub fn get_mode(&mut self) -> &Mode {
        &self.mode
    }
}
