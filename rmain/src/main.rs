#[link(name = "cfib", kind="static")]
extern {
    fn fib(n: i32) -> i32;
}

fn main() {
    let n = 7;
    println!("F({}) = {}", n, unsafe {fib(n)});
}
