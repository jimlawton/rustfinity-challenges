pub fn is_prime(n: u32) -> bool {
    // Implement your code here
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let upper: u32 = (n as f32).sqrt() as u32 + 1;
        for i in 3..upper {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

