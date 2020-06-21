use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
}