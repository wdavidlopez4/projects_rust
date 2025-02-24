fn main() {
    let number_texts: [&str; 12] = ["first", "second", "third", "fourth", "fifth", 
        "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let title = "On the XXXX day of Christmas";
    let my_love = "My true love gave to me";

    let body = "este es el curpo";

    let text_final = "A partridge in a pear tree";

    for (index, number_text) in number_texts.iter().enumerate(){

        let title_complet = title.replace("XXXX", number_text);
        let mut text_final_complet = text_final.replace("A", "And a");

        if index == 0 {
            text_final_complet =  text_final.to_string();
        }

        println!("{} \n {} \n {}  \n {}", title_complet, my_love, body, text_final_complet );
    }

}
