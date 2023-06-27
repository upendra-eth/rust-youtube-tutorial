fn main (){
    let mut my_string = String::from("helloworld  hsddjsdj jsdfjdsjj jjdsfhh");
    let x = first_word(&my_string); // x will get the value 5
    println!("first world ------ {x}");
    my_string.clear();  
    println!("first world ------ {x}");
 
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}