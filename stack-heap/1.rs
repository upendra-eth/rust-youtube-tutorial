fn main() {
    let a = 10; // Stack-allocated variable a
    let b = 20; // Stack-allocated variable b
    let sum = add_numbers(a, b); // Stack-allocated variable sum to store the result of adding a and b
    let heap_value = String::from("Hello, world!"); // Heap-allocated string
    println!("Sum: {}", sum);
    println!("Heap value: {}", heap_value);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y; // Stack-allocated variable result to store the result of adding x and y
    let heap_value2 = Box::new(vec![1, 2, 3, 4, 5]); 
    // Heap-allocated vector
    println!("Heap value 2: {:?}", *heap_value2);
}
