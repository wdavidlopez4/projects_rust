fn main() {
    let number_texts: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let title = "On the XXXX day of Christmas";
    let my_love = "My true love gave to me";

    let mut body = String::new();
    let two = "Two turtle doves";
    let three = "Three French hens";
    let four = "Four calling birds";
    let five = "Five golden rings";
    let six = "Six geese-a-laying";
    let seven = "Seven swans-a-swimming";
    let eight = "Eight maids-a-milking";
    let nine = "Nine ladies dancing";
    let ten = "Ten lords-a-leaping";
    let eleven = "Eleven pipers piping";
    let twelve = "Twelve drummers drumming";

    let text_final = "A partridge in a pear tree";

    for (index, number_text) in number_texts.iter().enumerate() {

        let title_complet = title.replace("XXXX", number_text);
        let mut text_final_complet = text_final.replace("A", "And a");
        body.clear();

        if index == 11{
            text_final_complet = text_final_complet.to_string() + 
            "\n" + 
            "And a partridge in a pear tree";
        }

        if index > 10 {
            body = body + twelve + "\n"
        }

        if index > 9 {
            body = body + eleven + "\n"
        }

        if index > 8 {
            body = body + ten + "\n"
        }

        if index > 7 {
            body = body + nine + "\n"
        }

        if index > 6 {
            body = body + eight + "\n"
        }

        if index > 5 {
            body = body + seven + "\n"
        }

        if index > 4 {
            body = body + six + "\n"
        }

        if index > 3 {
            body = body + five + "\n";
        }

        if index > 2 {
            body = body + four + "\n";
        }

        if index > 1 {
            body = body + three + "\n";
        }

        if index > 0 {
            body = body + two;
        }

        if index == 0 {
            text_final_complet = text_final.to_string();
        }

        println!(
            "{} \n {} \n {} \n {} \n \n",
            title_complet, my_love, body, text_final_complet
        );
    }
}
