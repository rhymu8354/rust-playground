fn main() {
    another_function(5 + 2, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The function 'five' returns this: {}", five());
}

fn five() -> i32 {
    5
}