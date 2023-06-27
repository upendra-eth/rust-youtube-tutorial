#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
let x =  build_user(String::from("uvsingh2336@gmail.com"),String::from("upendra"));
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("com"),
    ..x
};
 println!("user info {:#?}",user2 );
 println!("user info {:#?}",x );
}

fn build_user(email: String, username: String) -> User {
   
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }  
}

