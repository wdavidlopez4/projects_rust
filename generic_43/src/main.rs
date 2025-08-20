
struct Point<T, U>{
    x: T,
    y: U
}

impl<T, U> Point<T,U> {
    //metodos que implementan a las genericos de impl
    fn get_x(&self) -> &T{
        &self.x
    }

    fn set_x(&mut self, x: T){
        self.x = x
    } 

    fn get_y(&self) -> &U{
        &self.y
    }

    fn set_y(&mut self, y: U){
        self.y = y
    }

    //metodos que no implementan los genericos de impl
    fn mixup<V, W>(&self, point: Point<V, W>) -> Point<V, W>{
        Point { x: point.x, y: point.y }
    }
}


fn main() {
    let mut p = Point{
        x: 1,
        y: 2
    };

    p.set_x(2);
    p.set_y(4);
    
    let p2 : Point<i32, i32> = p.mixup(Point { x: 6, y: 8 });

    println!("p: {}, {}, p2: {} - {}", 
        p.get_x(), 
        p.get_y(), 
        p2.get_x(), 
        p2.get_y());
}
