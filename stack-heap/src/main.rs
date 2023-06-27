static mut COUNT: u32 = 0;
const PI: f32 = 3.14159;

fn main() {
    // Accessing static variable
    unsafe {
        println!("Initial count: {}", COUNT);
        COUNT += 1;
        println!("Updated count: {}", COUNT);
    }
    // Calling a function that modifies the static variable
    increment_count();
    // Accessing static variable again
    unsafe {
        println!("Final count: {}", COUNT);
    }
    println!("area of circle with radius 10 is {}", area_of_circle(10.0));
}
fn increment_count() {
    unsafe {
        COUNT += 1;
    }
}
fn area_of_circle(radius: f32) -> f32 {
    PI * radius * radius
}
