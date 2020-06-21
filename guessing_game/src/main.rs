// needed for gen_range
use rand::Rng; // -- Chapter 10 will explain why we needed this

fn main() {
    let mytuple = (2, 4, 6);

    println!("The second value in the tuple is {}.", mytuple.1);

    let mut map = std::collections::HashMap::new();
    map.insert(1, 10);
    map.insert(2, 42);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: isize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}