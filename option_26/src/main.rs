fn main() {
    let some_number = Some(5);
    let some_string = Some("algo");

    let absent_number:Option<i32> = None;

    println!("number: {:?}, str: {:?}, none: {:?} ", 
        some_number, some_string, absent_number);

}
