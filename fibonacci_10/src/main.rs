fn main() {
    for i in 1..=10{
        let fib = fibonacci(i);

        println!("{}, {}", i, fib);
    }
}

//poco eficiente por que re-computa los valores anteriores
fn fibonacci(number:i32) -> i32{
    if number <= 1{
        return number;
    }
    else{
        return fibonacci(number - 1) + fibonacci(number - 2);
    }
}
