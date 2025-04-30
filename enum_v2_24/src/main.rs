#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8), //4 valores enteros de 0 a 248 como es el protocolo ip v4
    V6(String)
}

fn main() {
    let v4 = IpAddr::V4(129, 0, 0, 1 ); 
    let v6 = IpAddr::V6(String::from("::1"));

    println!("{:?}, {:?}", v4, v6);
}
