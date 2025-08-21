
//trait
pub trait Summary{
    fn summarize(&self) -> String;
}

//struct
pub struct NewArtice{
    pub headline: String,
    pub location: String,
    pub autor: String
}

impl NewArtice { //impl sin trait sus propios metodos
    pub fn new(headline: String, location: String) -> NewArtice {
        NewArtice {
            headline,
            location,
            autor: String::from("william"),
        }
    }
}

impl Summary for NewArtice { //impl implementando trait
    fn summarize(&self) -> String {
        format!("{}, {} de {}", self.headline, self.location, self.autor)
    }
}


//main
fn main() {
    let s = NewArtice::new(
        String::from("Breaking News"),
        String::from("Colombia"),
    );

    println!("{}", s.summarize());
}
