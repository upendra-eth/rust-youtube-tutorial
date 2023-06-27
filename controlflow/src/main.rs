use std::{thread, time};
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
        thread::sleep(time::Duration::from_secs(1));

    }
    println!("LIFTOFF!!!");
}