fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("--------------{:?}----------{:?}--------",six,none)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(mut s) =>{ 
            s= s+20;
            Some(s) },
    }
}
