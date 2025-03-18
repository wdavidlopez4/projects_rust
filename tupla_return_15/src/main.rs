fn main() {
    let s1 = String::from("hola mundo");

    let (s, length) = len_string(s1);
    println!("tamano: {}, text: {}", s, length);
}

fn len_string(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}
