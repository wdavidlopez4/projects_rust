enum IdAddrKind{
    V4,
    V6
}

struct IdAddr{
    kind: IdAddrKind,
    address: String
}

fn main() {
    let home = IdAddr{
        kind: IdAddrKind::V4,
        address: String::from("home")
    };

    let loopback = IdAddr{
        kind: IdAddrKind::V6,
        address: String::from("work")
    };

    println!("enums: {} - {}", home.address, loopback.address);
}
