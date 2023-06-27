struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    fn error_managment(x: Option<i32>) -> Option<i32> {
        match x {
            None => {
                println!("return value is empty")
            },
            Some(i) => x,
        }
    }

}
