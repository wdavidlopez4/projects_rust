fn main() {
    /*
    ejemplo de estructura
     */

    struct User{
        user_name: String,
        email: String,
        sign_in_count: u64,
        active: bool
    };

    //inmutable
    let user1 = User{
        email: String::from("david@mail.com"),
        user_name: String::from("david"),
        active: true,
        sign_in_count: 1
    };

    println!("{}", user1.email);

    //mutable
    let mut user2 = User{
        email: String::from("david@mail.com"),
        user_name: String::from("david"),
        active: true,
        sign_in_count: 1
    };

    user2.email = String::from("david_new@gmail.com");
    println!("{}", user2.email);


    //funcion que retorna un usuario inmutable
    fn build_user(email: String, user_name: String) -> User {
        User {
            email: email,
            user_name: user_name,
            active: true,
            sign_in_count: 1 
        }
    }

    let user3 = build_user(String::from("william@"), String::from("william"));
    println!("{}", user3.email);


    //otra forma de escribir una funcion que retorne usuario
    fn build_user_2(email: String, user_name: String) -> User {
        User{
            email,
            user_name,
            active: true,
            sign_in_count: 1
        }
    }

    let user4 = build_user_2(String::from("william4@"), String::from("william"));
    println!("{}", user4.email);

    //sintaxis de actualizacion de estreuctura
    let user7 = User{
        email: String::from("wiliam7"),
        user_name: String::from("William7@"),
        active: true,
        sign_in_count: 1
    };

    let user8 = User{
        email: String::from("wiliam8"),
        user_name: String::from("William8@"),
        ..user7 //aqui le desimos que el resto de atributos se llene con user7
    };

    println!("{}", user8.email);

}