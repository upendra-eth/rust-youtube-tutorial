fn main () {
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem


// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{} and {}", r1, r2);
println!("{}", r3);

}
