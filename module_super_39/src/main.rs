mod instrument {
    fn clarinet(){
        super::breathe_in();
    }

    pub mod internal{
        pub fn funtion_internal(){
            super::clarinet();
        }
    }
}

fn breathe_in(){
    println!("estoy en el modulo por defaul: crate")
}

fn main() {
    crate::instrument::internal::funtion_internal();
}
