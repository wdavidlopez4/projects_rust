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

}