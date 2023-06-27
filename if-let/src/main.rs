
#[derive(PartialEq)]
enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

fn main() {
    println!("Hello, user!");
    let animal = Animal::Cat(String::from("sera"));
    IfLet(animal);
    // IfElse(animal);
    // MatchIt(animal);
}

fn MatchIt(animal: Animal){
    match animal {
        Animal::Dog(name) => println!("It's a dog named {}", name),
        Animal::Cat(name) => println!("It's a cat named {}", name),
        Animal::Bird(name) => println!("It's a bird named {}", name),
    }
      
}

fn IfLet (animal: Animal){
    if let Animal::Dog(name) = animal {
        println!("It's a dog named {}", name);
    }       
}
