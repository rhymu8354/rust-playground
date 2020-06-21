#[no_mangle]
pub extern fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => { fib(n - 1) + fib(n - 2) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(0, fib(0));
        assert_eq!(1, fib(1));
        assert_eq!(1, fib(2));
    }

}
