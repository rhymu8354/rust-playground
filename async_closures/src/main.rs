fn main() {
    let mut name = String::from("Fred");
    let mut closure = || {
        name = String::from("Bob");
    };
    println!("Name is {}", name);
    name = String::from("Alice");
    println!("Name is {}", name);
    closure();
    println!("Name is {}", name);
}

// fn main() {
//     let mut name = String::from("Fred");
//     let mut closure = || {
//         name = String::from("Bob");
//     };
//     closure();
//     println!("Name is {}", name);
// }

// fn main() {
//     let name = std::cell::RefCell::new(String::from("Fred"));
//     let closure = || {
//         *name.borrow_mut() = String::from("Bob");
//     };
//     println!("Name is {}", name.borrow());
//     *name.borrow_mut() = String::from("Alice");
//     println!("Name is {}", name.borrow());
//     closure();
//     println!("Name is {}", name.borrow());
// }

// fn main() {
//     let name = std::cell::RefCell::new(String::from("Fred"));
//     let closure = || async {
//         *name.borrow_mut() = String::from("Bob");
//     };
//     println!("Name is {}", name.borrow());
//     *name.borrow_mut() = String::from("Alice");
//     println!("Name is {}", name.borrow());
//     futures::executor::block_on(closure());
//     println!("Name is {}", name.borrow());
// }
