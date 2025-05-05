fn main() {
    let some_number = Some(5);
    let some_string = Some("algo");

    let absent_number:Option<i32> = None;

    let is_some_number = some_number.is_some();
    let value_number = some_number.unwrap();

    println!("number: {:?}, str: {:?}, none: {:?}, is some number: {}, value number {}", 
        some_number, some_string, absent_number, is_some_number, value_number);
}
