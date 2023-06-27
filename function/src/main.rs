fn main() {
    println!("Hello, world!");
    sum_of_xandy(30 ,50);
    sum_of_xandy(60 ,70);

    another_function1();
    sum_of_xandy(62 ,70);
    sum_of_xandy(64 ,70);
}

// this function is used to sum two no.
fn sum_of_xandy(x : u32 ,y :u32) {
    let sum = x +y ;
    println!("sum of x and y  {sum}") //macro to print line 
}

fn another_function1() {
    println!("Another function.");
}








// printing measutrment
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}  {unit_label}"); // printing value
}

fn another_function() {
    println!("Another function.");
}

 