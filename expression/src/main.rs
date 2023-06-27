fn main() {
    println!("Hello, world!");
    another_function1();
    println!("sum of x and y {} {} ",sum_of_xandy(30 ,50),sum_of_xandy(300 ,50)) ;//macro to print line 
    let xyz = sum_of_xandy(30 ,50);
}

fn sum_of_xandy(x : u32 ,y :u32) -> u32 {
    let sum = x +y ;
    sum 
}

fn another_function1() {
    println!("Another function.");
}





