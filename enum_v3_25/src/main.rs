
/*
ejemplo de estructuras dentro de un enums
 */

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

/*
ejemplo de diferentes elementos dentro de un enums,
y los enums al igual que las estructuras pueden tener metodos
 */

 enum Message{
    Quit, //no tiene ningun dato asociado solo es un tipo
    Move {x : i32, y: i32}, //struct
    Write(String), //tiene asociado un string
    ChangeColor(i32, i32, i32) //asociada una tupla
 }

 impl Message{
    fn Call(&self){
        //mas adelante miramos como implementar el curpo
    }
 }

fn main() {
    let v4 = IPAddr::V4(V4Addr {name:  String::from("123.12.13.0")});
    println!("{:?}", v4);

    let ex = Message::Quit;
    ex.Call();
}
