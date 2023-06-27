fn main() {
    let a = 10; // Stack-allocated variable a
    let b = 20; // Stack-allocated variable b
    let c = add_numbers(a, b); // Stack-allocated variable c to store the result of adding a and b

    let heap_value = String::from("Hello, world!"); // Heap-allocated String

    let a = 5; // Data shadowing: a is redeclared in a new scope
    let d = subtract_numbers(c, a); // Stack-allocated variable d to store the result of subtracting a from c
    print_sum(c, d);

    // The memory for heap_value will be released automatically when it goes out of scope
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let sum = x + y; // Stack-allocated variable sum to store the result of adding x and y
    sum
} // Memory for variable sum is released when the add_numbers function ends

fn subtract_numbers(x: i32, y: i32) -> i32 {
    let result = x - y; // Stack-allocated variable result to store the result of subtracting y from x
    result
} // Memory for variable result is released when the subtract_numbers function ends

fn print_sum(value1: i32, value2: i32) {
    println!("Sum: {}", value1);
    println!("Difference: {}", value2);
} // Memory for parameters value1 and value2 is released when the print_sum function ends
