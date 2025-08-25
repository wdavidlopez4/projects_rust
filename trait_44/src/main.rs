
//trait
pub trait Summary{
    fn summarize(&self) -> String;

    fn sumarize_author(&self) -> String{
        String::from("more...")
    }

    fn sumarize_location(&self) -> &str{
        "more..."
    }
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

    pub fn noty(item: &impl Summary) -> String { //resibe por parametro un trait tipo summary
        let sumarize = item.summarize();
        format!("execute sumarize : {} ", sumarize)
    }

    pub fn some_function<T>(item : T) -> String where T : Summary{ //limitando con where
        let sumarize = item.summarize();
        format!("execute some sumarize : {} ", sumarize)
    }
}

impl Summary for NewArtice { //impl implementando trait
    fn summarize(&self) -> String { //implementar el metodo
        format!("{}, {} de {}", self.headline, self.location, self.autor)
    }

    fn sumarize_location(&self) -> &str { //sobrescribir el metodo
        &self.location
    }
}


//main
fn main() {
    let s = NewArtice::new(
        String::from("Breaking News"),
        String::from("Colombia"),
    );

    let noty = NewArtice::noty(&s);

    println!("{} - {} - {} - {}", 
        s.summarize(), 
        s.sumarize_author(), 
        s.sumarize_location(),
        noty);
}
