#[derive(Debug)]
struct V4Addr{
    name: String
}

#[derive(Debug)]
struct V6Addr{
    name: String
}

#[derive(Debug)]
enum IPAddr{
    V4(V4Addr),
    V6(V6Addr)
}

fn main() {
    let v4 = IPAddr::V4(V4Addr {name:  String::from("123.12.13.0")});

    println!("{:?}", v4);
}
