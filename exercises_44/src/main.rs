
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
}

fn main() {
    let list_number: Vec<i32> = vec![1, 3 , 4, 5, 2, 1, 9, 1, 1];

    let mut statistic = Statistic::new();
    statistic.set_average(&list_number);

    println!("este es el promedio: {}", statistic.get_average())

}
