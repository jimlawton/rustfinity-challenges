pub fn fibonacci(n: u32) -> u32 {
    // TODO: Implement the Fibonacci sequence
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

