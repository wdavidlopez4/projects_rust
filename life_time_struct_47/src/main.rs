
/*
    quiere decir que solamente existe ImportantExcerpt si existe part,
    si deje de existe part entonces ImportantExcerpt ya no es valido
 */
struct ImportantExcerpt<'a>{
    part: &'a str
}

fn main() {
    let movel = String::from("Call me Ishmael. Some years ago...");
    
    let first_sentence = movel
        .split('.')
        .next()
        .expect("no se encuntra el .");

    let i = ImportantExcerpt{
        part : first_sentence
    };

    println!("{}", i.part);

}
