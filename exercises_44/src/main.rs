
struct Statistic{
    average : f32
}

impl Statistic{
    fn new() -> Self{
        Statistic{ average: 0.0}
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
}

fn main() {
    let mut list_number: Vec<i32> = vec![8, 3, 4, 5, 2, 11, 9, 12, 1];

    let mut statistic = Statistic::new();
    statistic.set_average(&list_number);

    println!("este es el promedio: {}", statistic.get_average());

    let a = Statistic::bubble_sort(& mut list_number);

    println!("prueba ordenada {:?}", a);

}
