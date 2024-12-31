pub fn factorial(n: u32) -> u128 {
    // Implement your code here
    println!("{n}");
    let mut fact: u128 = 1;
    if n > 0 {
        for i in (1..n+1).rev() {
            fact *= i as u128;
        }
    }
    fact
}

